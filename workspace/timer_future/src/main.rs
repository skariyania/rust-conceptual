use std::task::Context;
use std::time::Duration;

use futures::task::{waker_ref, ArcWake};
use futures::{future::BoxFuture, Future, FutureExt};
use std::sync::{
    mpsc::{sync_channel, Receiver, SyncSender},
    Arc, Mutex,
};

use timer_future::TimerFuture;

/// task executor that receives task off a channel and runs them.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // take the future and if it has not yet completed
            // (is still Some), poll it in an attempt to complete it
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // Create a `LocalWaker` from the task itself
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);

                // `BoxFuture<T>` is a type alias for
                // `Pin<Box<dyn Future<Option = ()> + Send + 'static>>`.
                // We can get a `Pin<&mut dyn Future + Send + 'static>`
                // from it by calling the `Pin::as_mut` method.

                if future.as_mut().poll(context).is_pending() {
                    // We'are not done processing the future, so put it back
                    // in its task to be run again in the future.
                    *future_slot = Some(future);
                }
            }
        }
    }
}

/// `Spawner` spawns new future onto the channel.
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

/// A future that can reschedule itself to be polled by an `Executor`
struct Task {
    /// In-progress future that should be pushed to completion
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing task at once. However rust isn't smart
    /// enough to know that `future` is mutated from only one thread,
    /// so we need to use `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// handle to place task itself back onto the task queue.
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement wake by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued")
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUE_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUE_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer
    spawner.spawn(async {
        println!("spawn a new task");
        // Wait for out timer future to complete after two seconds
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // Drop the spawner early so that our executor knows it is finished and
    // wont receive more incoming task to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // this will print "spawn a new task", pause for 2 seconds,
    // and then print "done!".
    executor.run();
}
