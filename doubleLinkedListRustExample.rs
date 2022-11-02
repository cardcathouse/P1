use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Node<T: Copy>{
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Copy> Node<T> {
    pub fn new(value: T) -> Self{
        Node{
            value,
            next: None,
            prev: None,
        }
    }
}

impl<T: Copy> From<Node<T>> for Option<Rc<RefCell<Node<T>>>>{
    fn from(node: Node<T>) -> Self{
        Some(Rc::new(RefCell::new(node)))
    }
}

type NodePtr<T> = Rc<RefCell<Node<T>>>;

pub struct List<T:Copy>{
    head: Option<NodePtr<T>>,
    tail: Option<NodePtr<T>>,
}

impl<T:Copy> List<T>{
    pub fn new() ->Self{
        List {
            head: None,
            tail: None
        }
    }

    pub fn push_front(&mut self, value: T){
        let mut node = Node::new(value);
        match &mut self.head.take(){
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            },
            Some(current_head) => {
                node.next = Some(current_head.clone());
                self.head = node.into();
                if let Some(h) = &self.head {
                    current_head.borrow_mut().prev = Some(Rc::downgrade(&h));
                }
            }
        }
    }

    pub fn push_back(&mut self, value: T){
        let mut node = Node::new(value);

        //we take ownership value of tail and replace it's value with None
        match &mut self.tail.take(){
            None => {
                //If list is empty, head and tail are the same
                self.head = node.into();
                self.tail = self.head.clone();  //Increments head ref. counter by 1
            }
            Some(current_tail) => { //If we already have something in tail
                //New node's prev. value is weak reference to current tail
                node.prev = Some(Rc::downgrade(&current_tail));
                self.tail = node.into(); //copy new node into tail
                //assigning new node to be current_tail next node. using borrow_mut to mutuably borrow value of new node
                current_tail.borrow_mut().next = self.tail.clone(); 
            }
        } 
    }

    pub fn pop_back(&mut self) -> Option<T> {
        //As before, we make tail None
        match &mut self.tail.take() {
            //if none, return none
            None => None,
            Some(tail) => {
                //as we don't want tail to have anything before, we borrow_mut tail...
                let mut tail = tail.borrow_mut();
                //..and then take it's prev. value
                let prev = tail.prev.take();
                //check if prev was head.
                match prev {
                    None => {
                        //if prev was head, then head also needs to be None
                        self.head.take();
                    }
                    Some(prev) => {
                        //if prev value had something, it's next value should be None
                        let prev = prev.upgrade(); //upgrading from weak reference
                        if let Some(prev) = prev {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev);
                        }
                    }
                };
                Some(tail.value) //return value
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match &mut self.head.take() {
            None => None,
            Some(head) => {
                let mut head = head.borrow_mut();
                let next = head.next.take();
                match next { 
                    None => {
                        self.tail.take();
                    },
                    Some(next) => {
                        next.borrow_mut().prev = None;
                        self.head = Some(next);
                    }
                };
                Some(head.value)
            }
        }
    }
}



mod tests{
    use super::*;
    fn works_builds_list()
    {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        assert_eq!(list.pop_back(),Some(4));
        assert_eq!(list.pop_back(),Some(3));
        assert_eq!(list.pop_back(),Some(2));
        assert_eq!(list.pop_back(),Some(1));
        assert_eq!(list.pop_back(), None);
    }

    fn works_builds_list_front()
    {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);

        assert_eq!(list.pop_front(),Some(4));
        assert_eq!(list.pop_front(),Some(3));
        assert_eq!(list.pop_front(),Some(2));
        assert_eq!(list.pop_front(),Some(1));
        assert_eq!(list.pop_front(), None);
    }
}