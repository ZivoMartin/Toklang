use super::tokenizer::{Tokenizer, push_token};

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum TokenType {
    // Primitive Token
    Ident,  
    Number,
    Symbol,
    Operator,
    Keyword,
    
    // Group Token
    Line,

    SerieArg,
    Arg,

    InKeyword,
    Macro,
    Section,
    
    Expression,
    Value,
    SerieIdent,
    SerieString,
    String,
    SerieChar,
    ComplexChar,

    BackLine,
    ERROR,   
    
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Flag {
    Section,
    NoFlag
}

pub static KEYWORDS: &[&'static str; 5] = &["GROUPS", "TPRIMS", "CHARS", "in", "END"];
pub static SECTIONS: &[&'static str; 4] = &["DECLARE", "CHAR_RULES", "TPRIM_RULES", "GROUP_RULES"];
pub static MACROS: &[&'static str; 3] = &["DIGITS", "LETTERS", "OTHER"];
pub static OPERATORS: &[&'static str; 2] = &["||", "&&"];
pub static OPERATOR_COMPONENT: &[char; 2] = &['|', '&'];
pub static DEFAULT_GARBAGE_CHARACTER: &[char; 2] = &[' ', '\t'];
static PRIMITIVE_TOKENTYPE: &[TokenType; 5] = &[TokenType::Ident, TokenType::Symbol, TokenType::Number, TokenType::Operator, TokenType::Keyword];
pub static PARSING_ERROR: &str = "Syntax error";
pub type ContentType = (usize, usize);
pub static EMPTY_TOKEN: ContentType = (0, 0);

pub enum TokenizerMessage {
    Token(Token),
    End()
}


#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub content: ContentType,
    pub flag: Flag 
}

impl Token {
    pub fn new(token_type: TokenType, content: ContentType, flag: Flag) -> Token {
        Token{token_type, content, flag}
    }
    
}

impl Copy for TokenType {}

impl Copy for Flag {}

impl Clone for TokenType {
    fn clone(&self) -> TokenType {
        return *self
    }
}

impl Clone for Flag {
    fn clone(&self) -> Flag {
        return *self
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Path<'a> {
    pub path: Vec<&'a Node>,
}

impl<'a> Path<'a> {
    pub fn init(node: &'a Node) -> Path {
        Path{path: vec!(node)}
    }

    pub fn p_node(&self) -> &'a Node {
        self.path[0]
    }

    pub fn proke_travel_functions(&self, tokenizer: &Tokenizer, content: ContentType) {
        for node in self.path.iter().rev() {
            if node.travel_react.is_some() {
                (node.travel_react.unwrap())(tokenizer, node.type_token, content, node.flag)
            }
        }
    } 
}

#[derive(Debug)]
pub struct Node {
    pub type_token: TokenType,
    pub flag: Flag,
    pub groups: Vec<Node>, 
    pub sons: Vec<Node>,
    pub can_end: bool,
    pub constraints: (Vec::<&'static str>, bool),
    pub consider_garbage: bool,
    pub retry: i8,
    pub travel_react: Option::<fn(&Tokenizer, TokenType, ContentType, Flag)>
}


impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        other.type_token == self.type_token
    }
}

fn get_default_constraint(token_type: TokenType ) -> Vec<&'static str> {
    match token_type {
        TokenType::Operator => Vec::from(OPERATORS),
        TokenType::Keyword => Vec::from(KEYWORDS),
        _ => Vec::new()
    }
}


impl Node {

    fn check_son(self) -> Node{
        for son in self.sons.iter() {
            if !PRIMITIVE_TOKENTYPE.contains(&son.type_token) {
                println!("ERROR DURING THE BUILDING OF THE TREE:");
                panic!("{:?} was found on a branch of a {:?} when a primitive type was expected", son.type_token, self.type_token);
            }
        }
        for group in self.groups.iter() {
            if PRIMITIVE_TOKENTYPE.contains(&group.type_token) {
                println!("ERROR DURING THE BUILDING OF THE TREE:");
                panic!("{:?} was found on a branch of a {:?} when a group type was expected", group.type_token, self.type_token);
            }
        }
        self
    }

    /// Build a new node wich has to be builded.
    pub fn new(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>) -> Node {
        Node::new_c(type_token, groups, sons, get_default_constraint(type_token))
    }

    pub fn new_c_r(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>, constraints: Vec<&'static str>, depth: i8) -> Node {
        Node{type_token, flag: Flag::NoFlag, groups, sons, can_end: true, constraints: (constraints, true), consider_garbage: false, retry: depth, travel_react: None}.check_son()        
    }

    /// Build a leaf, a leaf has to be builded
    pub fn leaf(type_token: TokenType) -> Node {
        Node::new_end(type_token, Vec::new(), Vec::new())
    }

    /// Build a new node wich can end the building of the group.
    pub fn new_end(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>) -> Node {
        Node::new_end_c(type_token, groups, sons, get_default_constraint(type_token))
    }

    pub fn new_c(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>, constraints: Vec<&'static str>) -> Node {
        Node{type_token, flag: Flag::NoFlag, groups, sons, can_end: false, constraints: (constraints, true), consider_garbage: false, retry: -1, travel_react: None}.check_son()
    }

    pub fn leaf_c(type_token: TokenType, constraints: Vec<&'static str>) -> Node {
        Node::new_end_c(type_token, Vec::new(), Vec::new(), constraints)
    }

    pub fn new_end_c(type_token: TokenType, groups: Vec<Node>, sons: Vec<Node>, constraints: Vec<&'static str>) -> Node {
        Node{type_token, flag: Flag::NoFlag, groups, sons, can_end: true, constraints: (constraints, true), consider_garbage: false, retry: -1, travel_react: None}.check_son()
    }

    pub fn is_leaf(&self) -> bool {
        self.sons.is_empty() && self.groups.is_empty()
    }

    pub fn priv_const(mut self) -> Node {
        self.constraints.1 = false;
        self
    }

    pub fn constraint_satisfied(&self, c: &str) -> bool {
        let contains = self.constraints.0.contains(&c);
        self.constraints.0.is_empty() || contains && self.constraints.1 || !contains && !self.constraints.1
    }

    pub fn react(mut self, r: fn(&Tokenizer, TokenType, ContentType, Flag)) -> Node {
        self.travel_react = Some(r);
        self
    }

    pub fn push(mut self) -> Node {
        self.travel_react = Some(push_token);
        self
    }
    
    pub fn consider_garbage(mut self) -> Node {
        self.consider_garbage = true;
        self
    }

    pub fn set_flag(mut self, flag: Flag) -> Node {
        self.flag = flag;
        self
    }
}
