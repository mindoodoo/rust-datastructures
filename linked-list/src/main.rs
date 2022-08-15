mod list;

use std::path::Display;

use list::List;

// fn disp(head: &List) {
//     println!("Head value is : {}", head.get_msg());
// }

fn main() {
    let mut head = List::Empty;
    head.push("yo");
    
    println!("Value in head is : {}", head.peek().unwrap());
}
