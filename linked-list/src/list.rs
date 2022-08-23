use std::{iter::{IntoIterator, Iterator}, fmt::Display, rc::Rc};

pub struct List<T>
where
    T: Copy
{
    head: Option<ListNode<T>>
}

type NextBox<T> = Rc<ListNode<T>>;

#[derive(Clone)]
struct ListNode<T>
where
    T: Copy
{
    data: T,
    next: Option<NextBox<T>>
}

impl<T> ListNode<T>
where
    T: Copy
{
    fn new(data: T, next: Option<ListNode<T>>) -> Self {
        let mut next_node: Option<NextBox<T>> = None;
        
        if let Some(x) = next {
            next_node = Some(Rc::new(x));
        }

        ListNode { data: data, next: next_node }
    }
}

impl<T> List<T>
where
    T: Copy
{
    pub fn new() -> Self {
        List {
            head: None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(x) => Some(&x.data)
        }
    }

    pub fn push_front(&mut self, val: T) {
        self.head = match &self.head {
            None => Some(ListNode::new(val, None)),
            Some(x) => Some(ListNode::new(val, Some(x.clone())))
        }
    }



    // TO IMPLEMENT
    //
    // [X] new
    // [X] peek
    // [X] print_head
    // [X] push_front
    // [ ] Iterator trait
    // [ ] FromIterator trait
    // [ ] push_back
    // [ ] pop_front
    // [ ] pop_back
}

impl<T> List<T>
where
    T: Display,
    T: Copy
{
    pub fn print_head(&self) {
        let head = self.peek();

        match head {
            None => println!("print_head(): List is empty..."),
            Some(x) => println!("{}", x)
        }
    }
}
