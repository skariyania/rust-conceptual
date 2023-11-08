use averaged_collection::AveragedCollection;

fn main() {
    let mut number_list = AveragedCollection::new();
    number_list.add(1);
    number_list.add(2);
    number_list.add(3);
    number_list.add(4);
    println!("average value of list is: {}", number_list.average());
}
