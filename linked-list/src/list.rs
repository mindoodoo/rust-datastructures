use std::iter::{IntoIterator, Iterator};

pub enum List<T> {
    Empty,
    Tail { data: T },
    Node { data: T, next: Box<List<T>> },
}

impl<T> List<T>
where
    T: Copy,
{
    pub fn push(&mut self, new: T) {
        match self {
            List::Empty => *self = List::Tail { data: new },
            List::Tail { data } => {
                let next_box;
                let new_node = List::Node {
                    data: data,
                    next: Box::new(List::Tail { data: new }),
                };

impl List {
    pub fn get_msg(&self) -> &str {
        return &self.msg;
    }

    pub fn new(msg: &str) -> List {
        List {
            msg: msg.to_string(),
            next: None
        }
    }

    pub fn pop_front(self) -> Option<List> {
        match self.next {
            Some(x) => Some(*x),
            None => None,
        }
    }

    // pub fn pop_back(self) -> Option<List> {
    //     let mut head: Box<List>;
    //     let mut previous: Option<Box<List>> = None;
        
    //     loop {
    //         match &self.next {
    //             Some(x) => {
    //                 previous = Some(Box::new(self));
    //                 head = *x;
    //             },
    //             None => break,
    //         }
    //     };

    //     match previous {
    //         None => None,
    //         Some(x) => Some(*x)
    //     }
    // }

    pub fn push(self, msg: &str) -> List {
        let old_head = Box::new(self);
        let output = List {
            msg: msg.to_string(),
            next: Some(old_head)
        };

        return output;
    }

    // To implement :
        // Iterator
        // Pop Front
        // Pop Back
        // Find
}

// impl Iterator for ListIter {
//     type Item = List;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.next {
//             None => None,
//             Some(x) => Some(x),
//         }
//     }
// }

// impl IntoIterator for List {
//     type Item = List;
//     type IntoIter = ListIter;

//     fn into_iter(self) -> Self::IntoIter {
//         return self;
//     }
// }
