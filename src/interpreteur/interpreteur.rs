use super::include::*;

type Consumer<'a> = fn(&mut Interpreteur<'a>, &'a str, &'a str, &'a str, TokenType) -> ConsumeResult;

pub struct Interpreteur<'a> {
    text: &'a str,
    symb_types: HashMap<&'a str, &'a str>,
    token_types: HashMap<&'a str, Identity<'a>>,
    group_types: HashMap<&'a str, Identity<'a>>,
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
                    let name = name.trim();
                    self.token_types.insert(name, Identity::token(name));
                }
            }
            "GROUPS" => {
                for name in right.split(",") {
                    let name = name.trim();
                    self.group_types.insert(name, Identity::group(name));
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
                self.token_types.get_mut(left).unwrap().set_forest(forest)?
            },
            "in" => {
                let mut constraints = Vec::new();
                for constraint in right[1..right.len()-1].split(",") {
                    let constraint = constraint.trim();
                    constraints.push(&constraint[1..constraint.len()-1]);
                }
                self.token_types.get_mut(left).unwrap().set_constraints(constraints)?;
            },
            _ => return Err(format!("This operator isn't authorized here: {op}"))
         };
        Ok(())
    }

    fn group_rules_token(&mut self,  left: &'a str, op: &'a str, right: &'a str, token_type: TokenType) -> ConsumeResult {
        if !self.group_types.contains_key(left) {
            return Err(format!("The group token {left} doesn't exists."))
        }
        let forest = self.ptoken_building_tree(left, right)?;
        self.group_types.get_mut(left).unwrap().set_forest(forest)?;
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

    fn extract_root(&self, mut root: &'a str) -> (&'a str, bool, Vec::<&'a str>) {
        let mut is_end = false;
        let mut constraints = Vec::<&'a str>::new();
        let mut split = root.split("{");
        root = split.next().unwrap().trim();
        if let Some(args) = split.next() {
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
        (root, is_end, constraints)
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
    
    fn ptoken_building_tree(&self, name: &'a str, mut expr: &'a str) -> Result<Forest<'a>, String> {
        expr = expr.trim();
        let mut iter = expr.chars().peekable();
        let mut forest = Forest::new();
        while iter.peek().is_some() {
            let sub_expr: &str;
            (sub_expr, expr) = self.get_next_expr(expr, &mut iter, '|');
            let (root, mut rest) = self.get_next_expr(sub_expr, &mut sub_expr.chars().peekable(), '&');
            rest = rest.trim();
            let (root, is_end, node_constraints) = self.extract_root(root);
            let mut new_node = if rest.is_empty() {
                Node::Leaf(root)
            } else {
                Node::Node(root, is_end, self.ptoken_building_tree(name, rest)?)
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
        if first == '(' && c == ')' {
            expr = &expr[1..(res-1)];
            *iter = expr.chars().peekable();
            self.get_next_expr(expr, iter, stop_char)
        } else {
            (&expr[0..res].trim(), &expr[res..expr.len()])
        }
    }  
}
