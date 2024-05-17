use super::include::*;

use super::sections::{
    define_section::DefineSection,
    symbol_rules_section::SymbolRulesSection,
    tprim_rules_section::TPrimRulesSection,
    group_rules_section::GroupRulesSection,
};

pub struct Interpreteur<'a> {
    text: &'a str,
    symb_types: HashMap<&'a str, &'a str>,
    current_section: &'a str,
    sections: HashMap<&'a str, Box<dyn Section>>
}

impl<'a> Interpreteur<'a> {

    pub fn new<'b>(text: &'b str) -> Interpreteur<'b> {
        Interpreteur {
            text,
            current_section: "",
            sections: Interpreteur::build_section_map(),
            symb_types: HashMap::new(),
        }
    }

    pub fn new_token(&mut self, token: Token) -> ConsumeResult {
        match token.token_type {
            TokenType::Ident => self.new_ident(token),
            _ => panic!("Unexpected token: {:?}", token.token_type)
        }
        Ok(())
    }

    
    fn new_ident(&mut self, token: Token) {
        match token.flag {
            Flag::Section => println!("new section: {}", token.content),
            _ => todo!("New ident for current section")
        }
    }

    fn build_section_map() -> HashMap<&'static str, Box<dyn Section>> {
        let mut res = HashMap::new();
        res.insert("DEFINE", DefineSection::new());
        res.insert("CHAR_RULES", SymbolRulesSection::new());
        res.insert("TPRIM_RULES", TPrimRulesSection::new());
        res.insert("GROUP_RULES", GroupRulesSection::new());
        res
    }

    
    // fn consume_token(&mut self, token: Token) -> ConsumeResult {
    //     if self.request_in_treatment {
    //         self.request_treaters[self.current_treater].consume(&mut self.database, token)?;
    //     } else {
    //         self.current_treater = *self.keyword_link.get(&token.content).expect(&format!("Interpreteur: Unknow main keyword: {}", token.content));
    //         self.request_in_treatment = true;
    //     }
    //     Ok(())
    // }
    
    // fn end_request(&mut self) -> ConsumeResult {
    //     self.request_treaters[self.current_treater].end(&mut self.database)?;
    //     self.request_in_treatment = false;
    //     Ok(())
    // }
    
    // fn build_treaters() -> Vec<Box<dyn Request>> {
    //     vec!(CreateReq::new(), DropReq::new(), ResetReq::new(), InsertReq::new(), SelectReq::new(), SetReq::new(), DeleteReq::new())
    // }

    // fn build_keyword_link() -> HashMap::<String, usize> {
    //     let mut res = HashMap::<String, usize>::new();
    //     for (i, kw) in Vec::from(["CREATE", "DROP", "RESET", "INSERT", "SELECT", "SET", "DELETE"]).iter().enumerate() {
    //         res.insert(String::from(*kw), i);
    //     }
    //     res
    // }
    
}

