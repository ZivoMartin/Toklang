use super::include::*;

type TypeChar<'a> = &'a str;
type TokenForest<'a> = Vec::<TokenNode<'a>>;

fn merge_forests<'a>(f1: &mut TokenForest<'a>, f2: &mut TokenForest<'a>) {
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

struct TokenIdentity<'a> {
    forest: TokenForest<'a>,
    constraints: Vec::<&'a str>
}

impl<'a> TokenIdentity<'a> {

    fn new() -> TokenIdentity<'a> {
        TokenIdentity {
            forest: TokenForest::new(),
            constraints: Vec::new()
        }
    }

    fn set_constraints(&mut self, constraints: Vec::<&'a str>) {
        self.constraints = constraints;
    }
    
    fn set_forest(&mut self, forest: TokenForest<'a>) {
        self.forest = forest;
    }
    
}

#[derive(Debug)]
#[derive(Clone)]
pub enum TokenNode<'a> {
    Node(TypeChar<'a>, bool, TokenForest<'a>),
    Leaf(TypeChar<'a>)
}


impl<'a> TokenNode<'a> {

    fn typechar(&self) -> TypeChar<'a> {
        match self {
            TokenNode::Node(tc, _, _) => tc,
            TokenNode::Leaf(tc) => tc
        }
    }

    fn merge(&mut self, node: &mut TokenNode<'a>) {
        match self {
            TokenNode::Node(_, can_end, forest) => {
                match node {
                    TokenNode::Leaf(_) => *can_end = true,
                    TokenNode::Node(_, _, new_forest) => merge_forests(forest, new_forest)
                }
            },
            TokenNode::Leaf(_) => {
                match node {
                    TokenNode::Node(root, _, forest) => *self = TokenNode::Node(root, true, forest.to_vec()),
                    TokenNode::Leaf(_) => ()
                }
            }
        }
    }
    
}



type Consumer<'a> = fn(&mut Interpreteur<'a>, &'a str, &'a str, &'a str, TokenType) -> ConsumeResult;

pub struct Interpreteur<'a> {
    text: &'a str,
    symb_types: HashMap<&'a str, &'a str>,
    token_types: HashMap<&'a str, TokenIdentity<'a>>,
    group_types: HashMap<&'a str, &'a str>,
    current_section: &'a str,
    sections: HashMap<&'a str, Consumer<'a>>,
}

impl<'a> Interpreteur<'a> {

    pub fn new(text: &str) -> Interpreteur<'_> {
        Interpreteur {
            text,
            symb_types: HashMap::new(),
            token_types: HashMap::new(),
            group_types: HashMap::new(),
            current_section: "",
            sections: Interpreteur::build_section_map(),
        }
    }

    pub fn new_token(&mut self, token: Token) -> ConsumeResult {
        let (i, j) = token.content;
        let line = &self.text[i..j].trim();
        if line.starts_with("#") {
            self.current_section = &line[1..];
            Ok(())
        } else {
            let token_meth = *self.sections.get(self.current_section).expect(&format!("The section {} doesn't exist", self.current_section));
            let (left, op, right) = self.split_line(line);
            token_meth(self, left.trim(), op.trim(), right.trim(), token.token_type)
        }
    }

    fn split_line(&self, line: &'a str) -> (&'a str, &'a str, &'a str) {
        let mut space = false;
        let mut prev = 'a';  // Non signigicant character
        let mut i = 0;
        for c in line.chars() {
            match c {
                ' ' => space = true,
                'n' => if space && prev == 'i' {
                    return (&line[0..i-1], &line[i-1..i+1], &line[i+1..])
                },
                '=' => return (&line[0..i], &line[i..i+1], &line[i+1..]),
                _ => ()
            }
            i += 1;
            prev = c;
        }
        panic!("Failed to tokenize a line");
    }
    

    fn define_token(&mut self,  left: &'a str, _op: &'a str, right: &'a str, token_type: TokenType) -> ConsumeResult {
        match left {
            "CHARS" => {
                for name in right.split(",") {
                    self.symb_types.insert(name.trim(), "");
                }
            }
            "TPRIMS" => {
                for name in right.split(",") {
                    self.token_types.insert(name.trim(), TokenIdentity::new());
                }
            }
            "GROUPS" => {
                for name in right.split(",") {
                    self.group_types.insert(name.trim(), "");
                }
            }
            _ => return Err(format!("You can't define '{left}'"))
        };
        Ok(())
    }

    fn tprim_rules_token(&mut self,  left: &'a str, op: &'a str, right: &'a str, token_type: TokenType) -> ConsumeResult {
        if !self.token_types.contains_key(left) {
            return Err(format!("The primitve token {left} doesn't exists."))
        }
        match op {
            "=" =>{
                let forest = self.ptoken_building_tree(left, right)?;
                self.token_types.get_mut(left).unwrap().set_forest(forest)
            },
            "in" => {
                let mut constraints = Vec::new();
                for constraint in right[1..right.len()-1].split(",") {
                    let constraint = constraint.trim();
                    constraints.push(&constraint[1..constraint.len()-1]);
                }
                self.token_types.get_mut(left).unwrap().set_constraints(constraints);
            },
            _ => return Err(format!("This operator isn't authorized here: {op}"))
         };
        Ok(())
    }

    fn group_rules_token(&mut self,  left: &'a str, op: &'a str, right: &'a str, token_type: TokenType) -> ConsumeResult {
        Ok(())
    }
    
    fn symb_rules_token(&mut self,  left: &'a str, _op: &'a str, right: &'a str, token_type: TokenType)  -> ConsumeResult {
        if self.symb_types.contains_key(left) {
            self.symb_types.insert(left, &right[1..right.len()-1]);
            Ok(())
        } else {
            Err(format!("{left} is an undefined symbol type."))
        }
    }

    fn build_section_map() -> HashMap<&'a str, Consumer<'a>> {
        let mut res = HashMap::<&'a str, Consumer>::new();
        res.insert("DECLARE", Interpreteur::define_token);
        res.insert("CHAR_RULES",  Interpreteur::symb_rules_token);
        res.insert("TPRIM_RULES", Interpreteur::tprim_rules_token);
        res.insert("GROUP_RULES", Interpreteur::group_rules_token);
        res
    }

    fn extract_root(&self, mut root: &'a str, name: &'a str) -> Result<(&'a str, bool, Vec::<&'a str>), String> {
        let mut is_end = false;
        let mut constraints = Vec::<&'a str>::new();
        let is_self = root == name;
        let mut split = root.split("{");
        root = split.next().unwrap().trim();
        if let Some(args) = split.next() {
            if name == root {
                return Err(format!("You can't pass an argument to {name}"))
            }
            let args = &args[0..args.len()-1];
            let mut j = 0;
            let mut comma = false;
            for (i, c) in args.chars().enumerate() {
                if c == ',' && !comma {
                    (is_end, constraints) = self.match_node_arg(&args[j..i], constraints, is_end);
                    j = i+1;
                } else if c == '\"' {
                    comma = !comma;
                }
            }
            (is_end, constraints) = self.match_node_arg(&args[j..], constraints, is_end)
        }
        if !is_self && !self.symb_types.contains_key(root) {
            Err(format!("The symbol {root} doesn't exists")) 
        } else {
            Ok((root, is_end, constraints))
        }
    }

    fn match_node_arg(&self, mut arg: &'a str,
                      mut constraints: Vec<&'a str>,
                      mut is_end: bool
    ) -> (bool, Vec<&'a str>) {
        arg = arg.trim();
        match arg as &str {
            "END" => is_end = true,
            _ => constraints.push(&arg[1..arg.len()-1])
        }
        (is_end, constraints)
    }
    
    fn ptoken_building_tree(&self, name: &'a str, mut expr: &'a str) -> Result<TokenForest<'a>, String> {
        expr = expr.trim();
        let mut iter = expr.chars().peekable();
        let mut forest = TokenForest::new();
        while iter.peek().is_some() {
            let sub_expr: &str;
            (sub_expr, expr) = self.get_next_expr(expr, &mut iter, '|');
            let (root, mut rest) = self.get_next_expr(sub_expr, &mut sub_expr.chars().peekable(), '&');
            rest = rest.trim();
            let (root, is_end, node_constraints) = self.extract_root(root, name)?;
            let mut new_node = if rest.is_empty() {
                TokenNode::Leaf(root)
            } else {
                TokenNode::Node(root, is_end, self.ptoken_building_tree(name, rest)?)
            };
            let mut push_it = true;
            for node in forest.iter_mut() {
                if node.typechar() == root {
                    node.merge(&mut new_node);
                    push_it = false;
                    break;
                }
            }
            if push_it {
                forest.push(new_node)
            } 
        }
        Ok(forest)
    }

    fn get_next_expr(&self, mut expr: &'a str, iter: &mut Peekable<Chars<'a>>, stop_char: char) -> (&'a str, &'a str) {
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
            self.get_next_expr(expr, iter, stop_char)
        } else {
            (&expr[0..res].trim(), &expr[res..expr.len()])
        }
    }  
}
