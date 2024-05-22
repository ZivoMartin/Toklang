pub use crate::tokenizer::include::{Token, TokenType};
pub type ConsumeResult = Result<(), String>;
pub use std::collections::HashMap;
pub use std::str::Chars;
pub use std::iter::Peekable;

pub type TypeChar<'a> = &'a str;
pub type TypeGroup<'a> = &'a str;
pub type Forest<'a> = Vec::<Node<'a>>;

pub struct Identity<'a> {
    name: &'a str,
    forest: Forest<'a>,
    constraints: Option<Vec<&'a str>>,
}

impl<'a> Identity<'a> {

    pub fn token(name: &'a str) -> Identity<'a> {
        Identity{
            name,
            forest: Forest::new(),
            constraints: Some(Vec::new())
        }
    }

    pub fn group(name: &'a str) -> Identity<'a> {
        Identity{
            name,
            forest: Forest::new(),
            constraints: None
        }
    }

    pub fn set_constraints(&mut self, new_constraints: Vec::<&'a str>) -> Result<(), String>{
        if self.constraints.is_some(){
            self.constraints = Some(new_constraints);
            Ok(())
        } else {
            return Err(format!("You tried to assign constraints to a group, it's not supported by the language yet."))
        }
    }
    
    pub fn set_forest(&mut self, new_forest: Forest<'a>) -> Result<(), String> {
        self.forest = new_forest;
        Ok(())
    }
    
}

fn merge_node_forests<'a>(f1: &mut Forest<'a>, f2: &mut Forest<'a>) {
    for node_f2 in f2.iter_mut() {
        let mut push_it = true;
        for node_f1 in f1.iter_mut() {
            if node_f1.typechar() == node_f2.typechar() {
                push_it = false;
                node_f1.merge(node_f2);
                break
            }
        }
        if push_it {
            f1.push(node_f2.clone());
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Node<'a> {
    Node(TypeChar<'a>, bool, Forest<'a>),
    Leaf(TypeChar<'a>)
}


impl<'a> Node<'a> {

    pub fn typechar(&self) -> TypeChar<'a> {
        match self {
            Node::Node(tc, _, _) => tc,
            Node::Leaf(tc) => tc
        }
    }

    pub fn merge(&mut self, node: &mut Node<'a>) {
        match self {
            Node::Node(_, can_end, forest) => {
                match node {
                    Node::Leaf(_) => *can_end = true,
                    Node::Node(_, _, new_forest) => merge_node_forests(forest, new_forest)
                }
            },
            Node::Leaf(_) => {
                match node {
                    Node::Node(root, _, forest) => *self = Node::Node(root, true, forest.to_vec()),
                    Node::Leaf(_) => ()
                }
            }
        }
    }
    
}
