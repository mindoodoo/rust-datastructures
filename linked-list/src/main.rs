mod list;

use std::path::Display;

use list::List;

// fn disp(head: &List) {
//     println!("Head value is : {}", head.get_msg());
// }

fn main() {
    let mut head: List<i32> = List::new();

    head.print_head();

    head.push_front(32);

    head.print_head();
}

// A -> X -> Y -> Z