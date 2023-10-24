pub trait Summary {
    fn summarize(&self) -> String;
}

pub fn notify(item: &impl Summary) {
    println!("*---------Notification--------------------------*");
    println!("Breaking news! {}", item.summarize());
    println!("*-----------------------------------------------*");
    println!("\n");
}
