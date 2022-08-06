use std::iter::{
    IntoIterator, Iterator
};

pub struct List {
    msg: String,
    next: Option<Box<List>>
}

struct ListIter {
    msg: String,
    next: Option<Box<List>>
}

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

    pub fn pop_back(self) -> Option<List> {
        let mut head: Box<List> = Box::new(self);
        let mut previous: Option<Box<List>> = None;
        
        loop {
            match head.next {
                Some(x) => {
                    previous = Some(head);
                    head = x;
                },
                None => break,
            }
        };

        match previous {
            None => None,
            Some(x) => Some(*x)
        }
    }

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

impl Iterator for ListIter {
    type Item = List;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(x) => Some(x),
        }
    }
}

impl IntoIterator for List {
    type Item = List;
    type IntoIter = ListIter;

    fn into_iter(self) -> Self::IntoIter {
        return self;
    }
}