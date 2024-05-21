use std::collections::HashMap;
use std::collections::VecDeque;
use crate::interpreteur::stack::Stack;
use super::include::*;
use super::grammar_tree::build_grammar_tree;
use std::iter::Peekable;
use std::str::Chars;
use std::sync::mpsc::Sender;

static COM_CHAR: char = '~';

pub struct Tokenizer {
    text: String,
    sender: Sender<TokenizerMessage>,                           // The thread asking the tokenization
    group_map: HashMap<TokenType, Node>,                       // Associate a token group to his node in the grammar tree
    priority_map: HashMap<TokenType, u8>,                      // Associate a primotive tokentype to his prority (keyword has a greater priority than an identificator)
    identity_map: HashMap<fn(char)->bool, Vec<TokenType>>      // Associate a function who recognize the signification of a char to the possible token type which could be built by this char 
}




fn build_priority_map() -> HashMap<TokenType, u8> {
    let mut priority_map = HashMap::<TokenType, u8>::new();
    priority_map.insert(TokenType::Ident, 1);
    priority_map.insert(TokenType::Number, 1);
    priority_map.insert(TokenType::Symbol, 2);
    priority_map.insert(TokenType::Operator, 1);
    priority_map.insert(TokenType::Keyword, 3);
    priority_map
}


fn build_identity_map() -> HashMap<fn(char)->bool, Vec<TokenType>> {
    let mut res = HashMap::<fn(char)->bool, Vec<TokenType>>::new();
    res.insert(is_number, vec!(TokenType::Number));
    res.insert(is_letter, vec!(TokenType::Ident, TokenType::Keyword));
    res.insert(is_sign, vec!(TokenType::Symbol, TokenType::Operator));
    res.insert(is_operator, vec!(TokenType::Operator, TokenType::Symbol));
    res
}

struct TextTraveler<'a> {
    save_stack: Stack<usize>,
    mark: usize,
    base_i: usize,
    i: usize,
    chars: Peekable<Chars<'a>>,
    text: &'a str,
}

impl<'a> TextTraveler<'a> {

    fn new(text: &'a str, base_i: usize) -> TextTraveler<'a> {
        TextTraveler {
            save_stack: Stack::new(),
            mark: 0,
            base_i,
            i: 0,
            chars: text.chars().peekable(),
            text
        }
    }

    fn mark(&mut self) {
        self.mark = self.i;
    }

    fn get(&self) -> &'a str {
        &self.text[self.mark..self.i]
    }

    fn get_msg(&self) -> ContentType {
        (self.mark+self.base_i, self.i+self.base_i)
    } 
    
    fn save(&mut self) {
        self.save_stack.push(self.i);
    }

    fn go_back(&mut self) {
        self.i = self.save_stack.pop().expect("Save stack empty");
        self.chars = self.text[self.i..].chars().peekable();
    }

    fn next(&mut self) -> Option<char> {
        self.i += (self.i < self.text.len()) as usize;
        self.chars.next()
    }

    fn peek(&mut self) -> Option<char> {
        match self.chars.peek() {
            Some(c) => Some(*c),
            None => None
        }
    }

    fn compute_next_line(&mut self) -> Option<(&str, usize)> {
        while let Some(c) = self.peek() {
            if !"\\\n \t".contains(c) {
                break;
            }
            self.next();
        }
        if self.peek() == None {
            return None;
        }
        self.mark();
        let mut last = '\n';
        while let Some(c) = self.next() {
            if c == '\n' && !"\\\n".contains(last) {
                break;
            }
            last = c;
        }
        Some((self.get().trim_end(), self.mark))
    }

    fn str_next(&mut self) -> &'a str {
        self.next();
        &self.text[self.i-1..self.i]
    }
}

impl<'a> Tokenizer {

    pub fn new(text: String, sender: Sender<TokenizerMessage>) -> Tokenizer {
        Tokenizer{
            text,
            sender,
            group_map: build_grammar_tree(),
            priority_map: build_priority_map(),
            identity_map: build_identity_map(),
        }
    }

    pub fn tokenize_file(&'a self) {
        let mut chars = TextTraveler::<'a>::new(&self.text, 0);
        while let Some((line, base_i)) = chars.compute_next_line() {
            if self.tokenize_one_line(line, base_i).is_err() {
                break;
            }
            push_token(self, TokenType::Line, (base_i, chars.i), Flag::NoFlag);
        }
        self.end();
    }

    fn end(&self) {
        let sender = self.sender.clone();
        sender.send(TokenizerMessage::End()).expect("Failed to send the tokenizer to main thread")
    }
    
    fn tokenize_one_line(&'a self, line: &'a str, base_i: usize) -> Result<(), ()>{
        let first_node = self.group_map.get(&TokenType::Line).unwrap();
        let mut chars = TextTraveler::<'a>::new(line, base_i);
        self.skip_garbage(&mut chars);
        while chars.peek().is_some() {
            if self.travel(first_node, &mut chars).is_err() {
                push_token(self, TokenType::ERROR, EMPTY_TOKEN, Flag::NoFlag);
                return Err(());
            }
            self.skip_garbage(&mut chars);
        }   
        Ok(())
    }
    
    fn travel(&'a self, current_node: &'a Node, chars: &mut TextTraveler) -> Result<(), i8> {
        if !current_node.is_leaf() {
            loop {
                let mut retry = false;
                if !current_node.consider_garbage {
                    self.skip_garbage(chars); 
                }
                if chars.peek().is_some() {
                    let mut paths_vec = self.get_son_array(current_node);
                    chars.save();
                    match self.get_next_token(&mut paths_vec, chars) {
                        Ok(token_string) => {
                            match self.filter_nodes(&mut paths_vec, &token_string) {
                               Some(path) => {
                                   path.proke_travel_functions(self, chars.get_msg());                                                   
                                   for node in path.path.iter() {
                                       match self.travel(node, chars) {
                                           Ok(_) => (),
                                           Err(depth) => {
                                               if current_node.retry != depth {
                                                   return Err(depth + 1)
                                               } 
                                               retry = true;
                                               break;
                                           }
                                       }
                                   }
                               }
                               _ => {
                                   chars.go_back();
                                   if !current_node.can_end {
                                       return Err(0)
                                   }
                               }
                           }
                        },
                        Err(_) => {
                            chars.go_back();
                            if !current_node.can_end {
                                return Err(0)
                            }
                        }
                    }
                }else if !current_node.can_end {
                    return Err(0);
                }
                if !retry {
                    break;
                }
            }
        }
        Ok(())
    }
    
    fn get_next_token(&self, path_vec: &mut VecDeque<Path>, chars: &mut TextTraveler<'a>) -> Result<&'a str, String> {
        let c = chars.peek().unwrap();
        chars.mark();
        if self.detect_char_token(path_vec, &c.to_string()) {            
            return Ok(chars.str_next()) 
        }
        for (cond_stop, author_type) in self.identity_map.iter() {
            if cond_stop(c) {
                if self.clean_son_vec(path_vec, author_type) {
                    self.next_char_while(chars, *cond_stop);
                    if chars.peek().is_some()
                        && *cond_stop == is_letter as fn(char)->bool
                        && is_number(chars.peek().unwrap())
                        && self.clean_son_vec(path_vec, &vec!(TokenType::Ident)) {  // If we are looking for an ident
                            self.next_char_while(chars, |c: char| {is_letter(c) || is_number(c)});
                    }
                    return Ok(chars.get())
                } else {
                    return Err(format!("FAILED TO TOKENIZE"))
                }
            }
        }
        Ok(chars.get())
    }

    fn detect_char_token(&self, path_vec: &mut VecDeque<Path>, c: &str) -> bool {
        let mut i = 0;
        while i < path_vec.len() {
            if path_vec[i].p_node().type_token == TokenType::Symbol && path_vec[i].p_node().constraint_satisfied(c){
                while path_vec.len() - 1 > i {
                    path_vec.pop_back();
                }
                while path_vec.len() != 1 {
                    path_vec.pop_front();
                }
                return true 
            }  
            i += 1;
        }
        false
    }

    fn clean_son_vec(&self, path_vec: &mut VecDeque<Path>, author_type: &Vec<TokenType>) -> bool {
        let mut i = 0;
        while i < path_vec.len() {
            if !author_type.contains(&path_vec[i].p_node().type_token) {
                path_vec.remove(i);
            }else{
                i += 1;
            }
        }
        !path_vec.is_empty()
    }

    fn next_char_while(&self, chars: &mut TextTraveler, continue_cond: fn(char)->bool) {
        chars.next();
        if continue_cond != is_sign as fn(char) -> bool {
            while let Some(c) = chars.peek() {
                if continue_cond(c) {    
                    chars.next();
                }else{
                    break;
                }
            }   
        }
    }

    fn get_son_array(&'a self, node: &'a Node) -> VecDeque<Path> {
        let mut res = VecDeque::<Path>::new();
        for son in node.sons.iter() {
            res.push_back(Path::init(son));
        }
        for group in node.groups.iter() {
            let mut paths = self.get_son_array(self.group_map.get(&group.type_token).unwrap());
            if group.travel_react.is_some() || !group.is_leaf() {
                for p in paths.iter_mut() {
                    p.path.push(group);
                }
            }
            res.append(&mut paths);
        }
        res
    }

    fn filter_nodes<'b>(&'a self, paths: &'a mut VecDeque::<Path>, token: &str) -> Option<&'b Path>{
        if token.is_empty() {
            return None
        }
        let mut i = 0;
        let mut res: Option<&Path> = None;
        while i < paths.len() {
            let node = paths[i].p_node();
            if node.constraint_satisfied(token) && (!res.is_some() || 
                self.priority_map.get(&res.unwrap().p_node().type_token) < self.priority_map.get(&node.type_token)){
                        res = Some(&paths[i])
            }
            i += 1;
        }
        res
    }

    fn skip_garbage(&self, chars: &mut TextTraveler) {
        while let Some(c) = chars.peek() {
            if c == COM_CHAR {
                while chars.next() != Some('\n') && chars.peek() != None {}
            }else{
                if !DEFAULT_GARBAGE_CHARACTER.contains(&c) {                  
                    break;
                }
                if c == '\n' {
                    push_token(self, TokenType::BackLine, EMPTY_TOKEN, Flag::NoFlag)
                }
                chars.next();
            }
        }
    }

}


pub fn push_token(tk: &Tokenizer, token_type: TokenType, content: ContentType, flag: Flag) {
    tk.sender.send(TokenizerMessage::Token(Token::new(token_type, content.clone(), flag))).expect("Error while sending new token");
}

fn is_sign(c: char) -> bool {
    !is_number(c) && !is_letter(c) && !DEFAULT_GARBAGE_CHARACTER.contains(&c) && !OPERATOR_COMPONENT.contains(&c)
}

fn is_number(c: char) -> bool {
    (c as u8) < 58 && (c as u8) >= 48
}

fn is_letter(c: char) -> bool {
    (c as u8) >= 65 && (c as u8) <= 122 && !((c as u8) >= 91 && (c as u8) <= 96) || c == '_'
}

fn is_operator(c: char) -> bool {
    OPERATOR_COMPONENT.contains(&c)
}

