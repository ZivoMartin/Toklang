use super::include::*;

pub struct Maps<'a> {
    symb_types: HashMap<&'a str, &'a str>
}

impl<'a> Maps<'a> {
    fn new() -> Maps<'a> {
        Maps{
            symb_types: HashMap::new()
        }
    }
}

pub struct Interpreteur<'a> {
    text: &'a str,
    maps: Maps<'a>,
    current_section: &'a str,
    sections: HashMap<&'a str, fn(&mut Interpreteur<'a>, Token)>
}

impl<'a> Interpreteur<'a> {

    pub fn new(text: &str) -> Interpreteur<'_> {
        Interpreteur {
            text,
            maps: Maps::new(),
            current_section: "",
            sections: Interpreteur::build_section_map(),
        }
    }

    pub fn new_token(&mut self, token: Token) -> ConsumeResult {
        match token.token_type {
            TokenType::Ident => self.new_ident(token),
            _ => self.consume_token(token)
        }
        Ok(())
    }

    
    fn new_ident(&mut self, token: Token) {
        let (i, j) = token.content;
        match token.flag {
            Flag::Section => self.current_section = &self.text[i..j],
            _ => self.consume_token(token)
        }
    }

    fn define_token(&mut self, token: Token) {
        
    }

    fn tprim_rules_token(&mut self, token: Token) {}
    
    fn group_rules_token(&mut self, token: Token) {}
    
    fn symb_rules_token(&mut self, token: Token) {}
    
    
    fn build_section_map() -> HashMap<&'a str, fn(&mut Interpreteur<'a>, Token)> {
        let mut res = HashMap::<&'a str, fn(&mut Interpreteur<'a>, Token)>::new();
        res.insert("DEFINE", Interpreteur::define_token);
        res.insert("CHAR_RULES",  Interpreteur::symb_rules_token);
        res.insert("TPRIM_RULES", Interpreteur::tprim_rules_token);
        res.insert("GROUP_RULES", Interpreteur::group_rules_token);
        res
    }


    fn consume_token(&mut self, token: Token) {
        let token_meth = *self.sections.get(self.current_section).expect(&format!("The section {} doesn't exist", self.current_section));
        token_meth(self, token);
    }
   
    
}

