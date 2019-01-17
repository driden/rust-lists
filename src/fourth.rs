use std::cell::RefCell;
use std::rc::Rc;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // head -> new_head -> old_head
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // there's going to be a new head in the list
                    new_head.borrow_mut().prev.take(); // prev = None
                    self.head = Some(new_head);
                }
                None => {
                    // need to empty the list
                    self.tail.take(); //tail = None
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| node.borrow().elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
// old_head :: Rc<RefCell<Node<T>>>
// type Link<T> = Option<Rc<RefCell<Node<T>>>>;
