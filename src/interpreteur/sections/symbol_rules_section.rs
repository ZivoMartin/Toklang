use crate::interpreteur::include::*;


pub struct SymbolRulesSection;


impl Section for SymbolRulesSection {

    fn new() -> Box<dyn Section> {
        Box::from(SymbolRulesSection)
    }

    fn new_token(&mut self, _token: Token) {}
    
}
