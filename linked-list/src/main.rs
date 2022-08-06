mod list;

use std::path::Display;

use list::List;

fn disp(head: &List) {
    println!("Head value is : {}", head.get_msg());
}

fn main() {
    let mut head = List::new("ye");
    head = head.push("yo");

    disp(&head);

    head = head.pop_back().unwrap();

    disp(&head);
}
