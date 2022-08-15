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

                *self = new_node;
            }
            List::Node { data, next } => next.push(),
        };
    }

    pub fn peek(&self) -> Option<T> {
        if let List::Empty = self {
            return None;
        }

        match *self {
            List::Empty => None,
            List::Tail { data } => Some(data),
            List::Node { data, .. } => Some(data),
        }
    }
}
