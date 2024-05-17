pub use crate::tokenizer::include::{Token, TokenType, Flag};
pub type ConsumeResult = Result<(), String>;
pub use std::collections::HashMap;
pub use std::str::Chars;
pub use std::iter::Peekable;


pub trait Section {

    fn new() -> Box::<dyn Section> where Self: Sized;
    
    fn new_token(&mut self, token: Token);
    
}


type TypeChar<'a> = &'a str;
type Forest<'a> = Vec::<Node<'a>>;

#[derive(Debug)]
pub enum Node<'a> {
    Node(TypeChar<'a>, bool, Forest<'a>),
    Leaf(TypeChar<'a>)
}

pub fn get_code(root: &str) -> &'static str {    
    "test"
}

pub fn ptoken_building_tree(mut expr: &str) -> Forest {
    expr = expr.trim();
    let mut iter = expr.chars().peekable();
    let mut forest = Forest::new();
    while iter.peek().is_some() {
        let sub_expr: &str;
        (sub_expr, expr) = get_next_expr(expr, &mut iter, '|');
        let (root, mut rest) = get_next_expr(sub_expr, &mut sub_expr.chars().peekable(), '&');
        rest = rest.trim();
        if rest.is_empty() {
            forest.push(Node::Leaf(get_code(root)))
        } else {
            forest.push(Node::Node(get_code(root), false, ptoken_building_tree(rest)));
        }
    }
    forest
}

fn get_next_expr<'a>(mut expr: &'a str, iter: &mut Peekable<Chars<'a>>, stop_char: char) -> (&'a str, &'a str) {
    let mut res = 0;
    let mut comma = false;
    let mut par_count = 0;
    let first = *iter.peek().expect("expr empty");
    let mut c = first;
    while let Some(new_char) = iter.next() {
        c = new_char;
        match c {
            '(' => par_count += 1,
            ')' => par_count -= 1,
            '\"' => comma = !comma,
            _ => {
                if c == stop_char && !comma && par_count == 0 {
                    let _ = iter.next();
                    return (&expr[0..(res-1)].trim(), &expr[(res+2)..expr.len()]);
                }
            }
        }
        if par_count == -1 {
            par_count = 0;
        }
        res += 1;
    }
    // res = res.trim().to_string();    
    if first == '(' && c == ')' {
        expr = &expr[1..(res-1)];
        *iter = expr.chars().peekable();
        get_next_expr(expr, iter, stop_char)
    } else {
        (&expr[0..res].trim(), &expr[res..expr.len()])
    }
}
