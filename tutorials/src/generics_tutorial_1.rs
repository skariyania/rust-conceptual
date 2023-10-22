use std::cmp::PartialOrd;
use std::fmt::Debug;

fn main() {
    let number_list = vec![1, 2, 10];
    let l1 = get_largest(&number_list);
    print_largest_num(l1);

    let another_list = vec![10, 290, 50];
    let l2 = get_largest(&another_list);
    print_largest_num(l2);

    let char_list = vec!['a', 'z', 'm'];
    let l3 = get_largest(&char_list);
    print_largest_num(l3);
}

fn get_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn print_largest_num<T: Debug>(num: &T) {
    println!("Largest number is {:?}", num);
}
