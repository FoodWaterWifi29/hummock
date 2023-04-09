// chain.rs
//
// use chain::Chain::*
//
// Implements a persistent list data structure with slicing and linking.
//

use std::rc::Rc;

enum ChainNode<T> {
    NonFinal(T, Rc<ChainNode<T>>),
    Final(T),
    Slice(Rc<ChainNode<T>>, u32),
    Link(Rc<ChainNode<T>>, Rc<ChainNode<T>>)
}

use ChainNode::*;

impl <T> ChainNode<T> {
    fn is_final(&self) -> bool {
        match &self {
            NonFinal(_, _) => false,
            Final(_) => true,
            Link(first, second) => false,
            Slice(_, 1) => true,
            Slice(_, _) => false
        }
    }

    fn head(&self) -> &T {
        match &self {
            NonFinal(head, _) => head,
            Final(head) => head,
            Link(first, _) => first.head(),
            Slice(start, _) => start.head()
        }
    }

    fn tail(&self) -> Rc<ChainNode<T>> {
        match &self {
            NonFinal(_, tail) => tail.clone(),
            Final(_) => panic!(),
            Link(start, end) =>
                if start.is_final() {
                    end.clone()
                } else {
                    Rc::new(Link(start.tail(), end.clone()))
                },
            Slice(start, size) => Rc::new(Slice(start.tail(), size - 1))
        }
    }
}

impl <T> std::cmp::PartialEq for ChainNode<T> where T: PartialEq {    
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

pub enum ChainState<'a, T> {
    NonEmptyChain(&'a T, Chain<T>),
    EmptyChain
}

pub use ChainState::*;

#[derive(Clone)]
pub struct Chain<T> {
    root: Option<Rc<ChainNode<T>>>,
    size: u32
}

impl <T> std::cmp::PartialEq for Chain<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size &&
        match (&self.root, &other.root) {
            (Some(node_ref), Some(other_node_ref)) => node_ref.as_ref() == other_node_ref.as_ref(),
            (None, None) => true,
            _ => false
        }
    }
}

impl <T> Chain<T> {
    pub const EMPTY: Chain<T> = Chain::<T> { root: None, size: 0 };

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn state(&self) -> ChainState<T> {
        match &self.root {
            Some(node_ref) => if node_ref.is_final() {
                NonEmptyChain(node_ref.head(), Chain::EMPTY)
            } else {
                NonEmptyChain(node_ref.head(), Chain { root: Some(node_ref.tail()), size: self.size - 1 })
            },
            None => ChainState::EmptyChain
        }
    }

    pub fn cons(item: T, tail: Chain<T>) -> Chain<T> {
        match tail.root {
            Some(node_ref) => Chain { root: Some(Rc::new(NonFinal(item, node_ref))), size: tail.size + 1 },
            None => Chain { root: Some(Rc::new(Final(item))), size: 1 }
        }
    }

    pub fn slice(start: Chain<T>, size: u32) -> Chain<T> {
        if start.size < size {
            panic!()
        }
        
        match start.root {
            Some(start_node_ref) => 
                Chain {
                    root: Some(Rc::new(Slice(start_node_ref.clone(), size))),
                    size: size
                },
            None => Chain::EMPTY
        }
    }

    pub fn concat(first: Chain<T>, second: Chain<T>) -> Chain<T> {
        match (&first.root, &second.root) {
            (Some(first_node_ref), Some(second_node_ref)) =>
                Chain {
                    root: Some(Rc::new(Link(first_node_ref.clone(), second_node_ref.clone()))),
                    size: first.size + second.size
                },
            (Some(_), None) => first,
            (None, Some(_)) => second,
            (None, None) => Chain::EMPTY
        }
    }
}

#[cfg(test)]
mod tests;
