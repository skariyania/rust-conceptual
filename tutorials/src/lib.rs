use std::thread;

pub fn public_function() {
    println!("hello from lib");
    another_private_function();
}

fn another_private_function() {
    println!("printed with another private fn inside library");
}

pub mod front_of_house;
pub mod lifetimes;
pub mod media_aggrigator;
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}

pub fn threading_example() {
    let list = vec![1, 2, 3];
    println!("before defining closure list values: {:?}", list);
    thread::spawn(move || println!("from thread, list values: {:?}", list))
        .join()
        .unwrap();
}

pub fn closure_example_trait() {
    #[derive(Debug)]
    struct Rectangle {
        height: u32,
        width: u32,
    }
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 4,
        },
        Rectangle {
            width: 7,
            height: 14,
        },
    ];
    let mut sort_ops = vec![];

    println!("list values {:#?}", list);
    list.sort_by_key(|r| {
        sort_ops.push(r.width);
        r.width
    });

    println!("sorted list values {:#?}", list);
    println!("magic values {:#?}", sort_ops);
}

pub fn iterator_example() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("iterating next value {:?}", val);
    }

    let mapper = v1.iter().map(|x| x + 1);
    println!("Iterator with map x=>x+1: {:?}", mapper);
    let collector: Vec<i32> = mapper.collect();
    println!(
        "Iterator with map x=>x+1: after collecting values {:?}",
        collector
    );
}

mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}
