use std::{iter::{IntoIterator, Iterator}, fmt::Display, rc::Rc};

type NextBox<T> = Rc<ListNode<T>>;

pub struct List<T>
where
    T: Copy
{
    head: Option<Rc<ListNode<T>>>,
}

#[derive(Clone)]
struct ListNode<T>
where
    T: Copy
{
    data: T,
    next: Option<NextBox<T>>
}

pub struct ListIterator<T>
where
    T: Copy
{
    current: Option<NextBox<T>>
}

impl<T> ListNode<T>
where
    T: Copy
{
    fn new(data: T, next: Option<Rc<ListNode<T>>>) -> Self {
        let mut next_node: Option<NextBox<T>> = None;
        
        if let Some(x) = next {
            next_node = Some(x.clone());
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
            None => Some(Rc::new(ListNode::new(val, None))),
            Some(x) => Some(Rc::new(ListNode::new(val, Some(x.clone()))))
        }
    }

    pub fn push_back(&mut self, val: T) {

    }

    // Note : Rc::new() poses issue as if pop_front is called after iter() creation
    // the pop_front change will not be reflected
    pub fn iter(&self) -> ListIterator<T> {
        match &self.head {
            None => ListIterator { current: None },
            Some(x) => ListIterator { current: Some(x.clone()) }
        }
    }

    // TO IMPLEMENT
    //
    // [X] new
    // [X] peek
    // [X] print_head
    // [X] push_front
    // [X] Iterator trait
    // [ ] push_back
    // [ ] pop_front
    // [ ] pop_back
}

impl<T> Iterator for ListIterator<T>
where
    T: Copy
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.clone();

        self.current = match &self.current {
            None => None,
            Some(x) => x.next.clone()
        };

        match current {
            None => None,
            Some(x) => Some(x.data)
        }
    }
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
