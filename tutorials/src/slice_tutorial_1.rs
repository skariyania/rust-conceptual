fn main() {
    let number_list = vec![1, 2, 10];
    let l1 = largest_number(&number_list);
    print_largest_num(l1);
    let another_list = vec![10, 290, 50];
    let l2 = largest_number(&another_list);
    print_largest_num(l2);
}

fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn print_largest_num(num: &i32) {
    println!("Largest number is {num}");
}
