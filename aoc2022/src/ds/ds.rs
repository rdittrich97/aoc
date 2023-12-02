#![allow(unused, dead_code)]

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};
fn binary_search<T: PartialOrd + std::fmt::Display>(src: &Vec<T>, key: &T) -> Option<usize> {
    let mut r: usize = src.len() - 1;
    let mut l: usize = 0;
    let mut m: usize;
    let mut val: Option<usize> = None;

    while l != r {
        m = (r - l) / 2 + l;
        if key == &src[m] {
            val = Some(m);
            break;
        } else if key > &src[m] {
            l = m + 1;
        } else {
            r = m;
        }
    }
    if &src[l] == key {
        val = Some(l);
    }
    return val;
}

fn bubble_sort<T: PartialOrd + std::fmt::Display + Copy>(src: &mut Vec<T>) {
    for i in 0..src.len() {
        for j in i..src.len() {
            if src[i] > src[j] {
                let t = src[i];
                src[i] = src[j];
                src[j] = t;
            }
        }
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug, Clone)]
struct Node<T: std::fmt::Debug + Clone> {
    next: Link<T>,
    prev: Link<T>,
    val: T,
}

impl<T: std::fmt::Debug + Clone> Node<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        return Rc::new(RefCell::new(Node {
            val: val,
            next: None,
            prev: None,
        }));
    }
}

#[derive(Debug)]
pub struct LList<T: std::fmt::Debug + Clone> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}

impl<T: std::fmt::Debug + Clone> LList<T> {
    pub fn get(&self, idx: usize) -> Option<T> {
        let mut ptr = self.head.clone();
        for _i in 0..idx {
            match ptr {
                Some(v) => {
                    ptr = v.borrow().next.clone();
                    println!("{:?}", v.borrow().val.clone());
                }
                None => {
                    break;
                }
            }
        }

        match ptr {
            Some(v) => {
                return Some(v.borrow().val.clone());
            }
            None => {
                return None;
            }
        }
    }
    pub fn length(&self) -> usize {
        return self.length;
    }

    pub fn append(&mut self, val: T) {
        let nn = Node::new(val);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(nn.clone());
                nn.borrow_mut().prev = Some(old_tail);
                self.tail = Some(nn);
            }
            None => {
                self.head = Some(nn.clone());
                self.tail = Some(nn)
            }
        }
        self.length += 1;
    }

    pub fn prepend(&mut self, val: T) {
        let nn = Node::new(val);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(nn.clone());
                nn.borrow_mut().next = Some(old_head);
                self.head = Some(nn);
            }
            None => {
                self.head = Some(nn.clone());
                self.tail = Some(nn)
            }
        }
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(old_head) => {
                match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev.take();
                        self.head = Some(new_head);
                    }
                    None => {
                        self.tail.take();
                    }
                }
                return Some(Rc::try_unwrap(old_head).ok().unwrap().into_inner().val);
            }
            None => {
                return None;
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(old_tail) => {
                match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                    }
                    None => {
                        self.head.take();
                    }
                }
                return Some(Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val);
            }
            None => {
                return None;
            }
        }
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        match self.head.as_ref() {
            Some(node) => return Some(Ref::map(node.borrow(), |node| &node.val)),
            None => return None,
        }
    }

    pub fn new() -> Self {
        return LList {
            head: None,
            tail: None,
            length: 0,
        };
    }

    pub fn print(&self) {
        let mut ptr = self.head.clone();
        for _i in 0..self.length {
            match ptr {
                Some(v) => {
                    ptr = v.borrow().next.clone();
                    print!("{:?}", v.borrow().val.clone());
                    if ptr.is_some() {
                        print!(" -> ");
                    }
                }
                None => {
                    break;
                }
            }
        }
        println!();
    }
}

impl<T: std::fmt::Debug + Clone> Drop for LList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
