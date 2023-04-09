// list.rs
//
// use list::List::*
//
// Implements a persistent list data structure. A stripped down implementation of Chain.
//

use std::rc::Rc;

enum ListNode<T> {
    NonFinal(T, Rc<ListNode<T>>),
    Final(T)
}

use ListNode::*;

impl <T> ListNode<T> {
    fn is_final(&self) -> bool {
        match &self {
            NonFinal(_, _) => false,
            Final(_) => true
        }
    }

    fn head(&self) -> &T {
        match &self {
            NonFinal(head, _) => head,
            Final(head) => head
        }
    }

    fn head_move(self) -> T {
        match self {
            NonFinal(head, _) => head,
            Final(head) => head
        }
    }

    fn tail(&self) -> Rc<ListNode<T>> {
        match &self {
            NonFinal(_, tail) => tail.clone(),
            Final(_) => panic!()
        }
    }
}

impl <T> std::cmp::PartialEq for ListNode<T> where T: PartialEq {    
    fn eq(&self, other: &Self) -> bool {
        match (self.is_final(), other.is_final()) {
            (true, true) => self.head() == other.head(),
            (false, false) => if self.head() == other.head() {
                self.tail().eq(&other.tail())
            } else {
                false
            },
            _ => false
        }
    }
}

pub enum ListState<'a, T> {
    NonEmptyList(&'a T, List<T>),
    EmptyList
}

pub use ListState::*;

pub struct List<T> {
    root: Option<Rc<ListNode<T>>>
}

impl<T> List<T> {
    pub const EMPTY: List<T> = List::<T> { root: None };

    pub fn is_empty(&self) -> bool {
        if let None = self.root { true } else { false }
    }

    pub fn state(&self) -> ListState<T> {
        match &self.root {
            Some(node_ref) => if node_ref.is_final() {
                NonEmptyList(node_ref.head(), List::EMPTY)
            } else {
                NonEmptyList(node_ref.head(), List { root: Some(node_ref.tail()) })
            },
            None => ListState::EmptyList
        }
    }

    pub fn cons(item: T, tail: List<T>) -> List<T> {
        match tail.root {
            Some(node_ref) => List { root: Some(Rc::new(NonFinal(item, node_ref))) },
            None => List { root: Some(Rc::new(Final(item))) }
        }
    }

    pub fn reverse_into_vec(self) -> Vec<T>
        where T: Clone
    {
        let mut old_list: List<T> = self;
        let mut result: Vec<T> = Vec::new();
        loop {
            match old_list.state() {
                NonEmptyList(head, tail) => {
                    result.push(head.clone());
                    old_list = tail;
                },
                EmptyList => break
            }
        }

        result
    }
}

impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        return Self{ root: self.root.clone() }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self{ List::EMPTY }
}
