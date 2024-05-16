use std::collections::HashMap;
use std::collections::VecDeque;
use super::include::*;
use super::grammar_tree::build_grammar_tree;
use std::iter::Peekable;
use std::fs::File;
use std::str::Chars;
use std::io::prelude::*;
use std::sync::mpsc::Sender;

static COM_CHAR: char = '#';

pub struct Tokenizer {
    sender: Sender<TokenizerMessage>,                          // The thread asking the tokenization
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
    priority_map.insert(TokenType::Type, 2);
    priority_map.insert(TokenType::Keyword, 3);
    priority_map
}


fn build_identity_map() -> HashMap<fn(char)->bool, Vec<TokenType>> {
    let mut res = HashMap::<fn(char)->bool, Vec<TokenType>>::new();
    res.insert(is_number, vec!(TokenType::Number));
    res.insert(is_letter, vec!(TokenType::Ident, TokenType::Type, TokenType::Keyword));
    res.insert(is_sign, vec!(TokenType::Symbol, TokenType::Operator));
    res.insert(is_operator, vec!(TokenType::Operator, TokenType::Symbol));
    res
}

fn compute_next_query(chars: &mut Peekable<Chars>) -> Result<String, String> {
    let mut query = String::new();
    let mut last = ';';
    let mut comma = false;
    while let Some(c) = chars.next() {
        last = c;
        query.push(c);
        if c == ';' && !comma {
            break;
        }
        if c == '\'' {
            comma = !comma;
        }
    }
    query = query.trim().to_string();
    if last != ';' && !query.is_empty() {
        eprintln!("ERROR: You forgot a comma");
        Err(query)
    } else {
        Ok(query)
    }
}

impl<'a> Tokenizer {

    pub fn new(sender: Sender<TokenizerMessage>) -> Tokenizer {
        Tokenizer{
            sender,
            group_map: build_grammar_tree(),
            priority_map: build_priority_map(),
            identity_map: build_identity_map(),
        }
    }

    pub fn tokenize_file(mut self, path: String) {
        let mut file = File::open(&path).expect(&format!("File {} doesn't exists", path));
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();
        match self.precompile(file_content, &path) {
            Ok(s) => {
                let mut chars = s.chars().peekable();
                while chars.peek().is_some() {
                    let query = compute_next_query(&mut chars);
                    if query.is_err() || self.tokenize_one_query(query.unwrap()).is_err() {
                        break;
                    }
                }
            }
            Err(e) => push_token(&self, TokenType::ERROR, &e, Flag::NoFlag)
        }
        self.end()
    }

    pub fn tokenize_query(mut self, query: String) {
        let _ = self.tokenize_one_query(query);
        self.end()
    }

    fn end(self) {
        let sender = self.sender.clone();
        sender.send(TokenizerMessage::Tokenizer(self)).expect("Failed to send the tokenizer to main thread")
    }
    
    fn tokenize_one_query(&mut self, query: String) -> Result<(), ()>{
        let first_node = self.group_map.get(&TokenType::Request).unwrap();
        let mut chars = query.chars().peekable();
        self.skip_garbage(&mut chars);
        while chars.peek().is_some() {
            if self.travel(first_node, &mut chars).is_err() {
                push_token(self, TokenType::ERROR, &FAIL_MESSAGE.to_string(), Flag::NoFlag);
                return Err(());
            }
            self.skip_garbage(&mut chars); 
        }   
        Ok(())
    }
    
    fn travel(&self, current_node: &'a Node, chars: &mut Peekable<Chars>) -> Result<(), i8> {
        if !current_node.is_leaf() {
            loop {
                let mut retry = false;
                if !current_node.consider_garbage {
                    self.skip_garbage(chars); 
                }
                if chars.peek().is_some() {
                    let mut paths_vec = self.get_son_array(current_node);
                    let save = chars.clone();
                    match self.get_next_token(&mut paths_vec, chars) {
                        Ok(token_string) => {
                            match self.filter_nodes(&mut paths_vec, &token_string) {
                                Some(path) => {
                                    path.proke_travel_functions(self, &token_string);                                                   
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
                                    *chars = save;
                                    if !current_node.can_end {
                                        return Err(0)
                                    }
                                }
                            }
                        },
                        Err(_) => {
                            *chars = save;
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

    fn get_next_token(&self, path_vec: &mut VecDeque<Path>, chars: &mut Peekable<Chars>) -> Result<String, String> {
        let c = chars.peek().unwrap();
        if self.detect_char_token(path_vec, &c.to_string()) {
            return Ok(chars.next().unwrap().to_string()) 
        }
        let mut current_token = String::new();
        for (cond_stop, author_type) in self.identity_map.iter() {
            if cond_stop(*c) {
                if self.clean_son_vec(path_vec, author_type) {
                    self.next_char_while(&mut current_token, chars, *cond_stop);
                    if chars.peek().is_some()
                        && *cond_stop == is_letter as fn(char)->bool
                        && is_number(*chars.peek().unwrap())
                        && self.clean_son_vec(path_vec, &vec!(TokenType::Ident)) {  // If we are looking for an ident
                            self.next_char_while(&mut current_token, chars, |c: char| {is_letter(c) || is_number(c)});
                    }
                    return Ok(current_token)
                }else{
                    return Err(format!("FAILED TO TOKENIZE"))
                }
            }
        }
        Ok(current_token)
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

    fn next_char_while(&self, current_token: &mut String, chars: &mut Peekable<Chars>, continue_cond: fn(char)->bool) {
        current_token.push(chars.next().unwrap());
        if continue_cond != is_sign as fn(char) -> bool {
            while let Some(c) = chars.peek() {
                if continue_cond(*c) {    
                    current_token.push(chars.next().unwrap());
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

    fn filter_nodes(&'a self, paths: &'a mut VecDeque::<Path>, token: &str) -> Option<&Path>{
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

    fn skip_garbage(&self, chars: &mut Peekable<Chars>) {
        while let Some(c) = chars.peek() {
            if *c == COM_CHAR {
                while chars.next() != Some('\n') && chars.peek() != None {}
            }else{
                if !DEFAULT_GARBAGE_CHARACTER.contains(c) {                  
                    break;
                }
                if *c == '\n' {
                    push_token(self, TokenType::BackLine, &String::new(), Flag::NoFlag)
                }
                chars.next();
            }
        }
    }

    fn precompile(&self, input: String, current_file_path: &str) -> Result<String, String> {
        let mut vec: Vec<String> = input.split("$").map(String::from).collect();
        let mut annalyse_next = true;
        let mut iter = vec.iter_mut();
        iter.next();
        while let Some(s) = iter.next() {
            if annalyse_next {
                let mut chars = s.chars();
                let mut current_line = String::new();
                while let Some(c) = chars.next() {
                    if c == '\n' {
                        break;
                    }
                    current_line.push(c);
                }
                *s = self.new_precompile_macro(current_line, &current_file_path)? + &chars.collect::<String>();
            }else{
                *s = String::from("$") + s;
            }
            annalyse_next = !s.ends_with("\\");
        }
        Ok(vec.join(""))
    }

    fn new_precompile_macro(&self, m: String, _current_file_path: &str) -> Result<String, String> {
        let split: Vec<&str> = m.split_whitespace().collect();
        match &split[0] as &str {
            _ => Err(format!("Unknow token: {}", split[0]))
        }

    }

}


pub fn push_token(tk: &Tokenizer, token_type: TokenType, content: &String, flag: Flag) {
    tk.sender.send(TokenizerMessage::Token(Token::new(token_type, content.clone(), flag))).expect("Error while sending new token");
}

pub fn end_request(tk: &Tokenizer, _token_type: TokenType, _content: &String, _flag: Flag) {
    push_token(tk, TokenType::End, &String::new(), Flag::NoFlag)
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

