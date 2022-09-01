mod list;

use std::{fmt::Display};

use list::List;

// fn disp(head: &List) {
//     println!("Head value is : {}", head.get_msg());
// }

fn print_vec<T>(vector: Vec<T>)
where
    T: Display
{
    for value in vector {
        println!("{}", value);
    }
}

fn main() {
    //
    // Linked list iterator test
    //
    let mut head: List<i32> = List::new();
    head.push_front(1); // Push shit
    head.push_front(2);
    head.push_front(3);

    let iter = head.iter();

    head.push_front(4);

    print_vec(iter.collect());

    //
    // Vector iterator test
    //
    let mut vector1: Vec<i32> = Vec::new();    
    vector1.push(1); // Push shit
    vector1.push(2);
    vector1.push(3);

    let iter1 = vector1.iter();

    vector1.push(4);

    print_vec(iter1.collect());
}