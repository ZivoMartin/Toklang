use super::include::*;

type Consumer<'a> = fn(&mut Interpreteur<'a>, &'a str, TokenType) -> ConsumeResult;

pub struct Interpreteur<'a> {
    text: &'a str,
    symb_types: HashMap<&'a str, &'a str>,
    token_types: HashMap<&'a str, &'a str>,
    group_types: HashMap<&'a str, &'a str>,
    current_section: &'a str,
    sections: HashMap<&'a str, Consumer<'a>>,
    left_ident: &'a str,
    right_exp: Vec<&'a str>
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
            left_ident: "",
            right_exp: Vec::new()
        }
    }

    pub fn new_token(&mut self, token: Token) -> ConsumeResult {
        match token.token_type {
            TokenType::Ident => self.new_ident(token)?,
            _ => self.consume_token(token)?
        }
        Ok(())
    }

    
    fn new_ident(&mut self, token: Token) -> ConsumeResult {
        let (i, j) = token.content;
        match token.flag {
            Flag::Section => self.current_section = &self.text[i..j],
            _ => self.consume_token(token)?
        }
        Ok(())
    }

    fn get_map(&'a mut self, type_map: &'a str) -> Result<&'a mut HashMap<&'a str, a'a str>, String> {
        match type_map {
            "SYMB" => Ok(&mut self.symb_types),
            "TPRIM" => Ok(&mut self.token_types),
            "GROUPS" => Ok(&mut self.group_types),
            _ => Err(format!("Invalid indentificator: {type_map}"))
        }
    }
    

    fn define_token(&mut self, content: &'a str, token_type: TokenType) -> ConsumeResult {

        match token_type {
            TokenType::Keyword => self.left_ident = content,
            TokenType::Ident => self.get_map(self.left_ident).insert(content, ""),
            TokenType::BackLine => () 
        }
        Ok(())
    }

    fn tprim_rules_token(&mut self, content: &'a str, token_type: TokenType) -> ConsumeResult { 
        println!("{content}");
        Ok(())
    }
    
    fn group_rules_token(&mut self, content: &'a str, token_type: TokenType) -> ConsumeResult {
        println!("{content}");
        Ok(())
    }
    
    fn symb_rules_token(&mut self, content: &'a str, token_type: TokenType)  -> ConsumeResult {
        println!("{token_type:?} {content}");
        Ok(())
    }
    
    
    fn build_section_map() -> HashMap<&'a str, Consumer<'a>> {
        let mut res = HashMap::<&'a str, Consumer>::new();
        res.insert("DEFINE", Interpreteur::define_token);
        res.insert("CHAR_RULES",  Interpreteur::symb_rules_token);
        res.insert("TPRIM_RULES", Interpreteur::tprim_rules_token);
        res.insert("GROUP_RULES", Interpreteur::group_rules_token);
        res
    }


    fn consume_token(&mut self, token: Token) -> ConsumeResult {
        let (i, j) = token.content; 
        let token_meth = *self.sections.get(self.current_section).expect(&format!("The section {} doesn't exist", self.current_section));
        token_meth(self, &self.text[i..j], token.token_type)
    }
   
    
}

