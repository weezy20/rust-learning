use algo_rust::linked_list_enum::List;

fn main() {
    println!("Hello, world with algorithms and data structures!");
    let mut xs = List::new();
    xs.push(1);
    xs.push(2);
    println!("{:?}", xs.pop());
    println!("{:?}", xs.pop());
}
