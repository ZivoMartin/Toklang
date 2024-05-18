use crate::interpreteur::include::*;


pub struct SymbolRulesSection<'a> {
    text: &'a str
}


impl<'a> Section<'a> for SymbolRulesSection<'a> {

    fn new(text: &'a str) -> Box<dyn Section<'a> + 'a> {
        Box::from(SymbolRulesSection {text})
    }

    fn new_token(&mut self, maps: &mut Maps, _token: Token) {}
    
}
