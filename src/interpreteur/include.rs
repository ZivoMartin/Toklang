pub use crate::tokenizer::include::{Token, TokenType, Flag};
pub use std::process::exit;
pub use super::string_builder::StringBuilder;
pub use std::collections::HashMap;
pub type ConsumeResult = Result<(), String>;
pub use super::stack::Stack;
pub use std::str::Chars;
pub use std::iter::Peekable;

use std::path::{
    Path,
};

type TypeChar = usize;
type Forest = Vec::<Node>;


pub enum Node {
    Node(TypeChar, bool, Forest),
    Leaf(TypeChar)
}


pub fn ptoken_building_tree(mut expr: &mut Peekable<Chars>) -> Forest {
    let mut forest = Forest::new();
    setup_expr(&mut expr);
    while expr.peek().is_some() {
        let sub_exp = get_next_expr(&mut expr, '|').chars();
        let sub_exp = sub_exp.peekable();
        setup_expr(&mut sub_exp);
    }
    forest
}

fn setup_expr(expr: &mut Peekable<Chars>)  {
    if expr.peek() == Some(&'(') {
        let _ = expr.next();
    }
}

fn get_next_expr(expr: &mut impl Iterator<Item = char>, stop_char: char) -> String {
    let mut res = String::new();
    let mut comma = false;
    let mut par_count = 0;
    while let Some(c) = expr.next() {
        match c {
            '(' => par_count += 1,
            ')' => par_count -= 1,
            '\"' => comma = !comma,
            _ => {
                if c == stop_char && !comma && par_count == 0 {
                    let _ = expr.next();
                    return res;
                }
            }
        }
        res.push(c);
    }
    res
}
