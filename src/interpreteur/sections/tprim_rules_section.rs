use crate::interpreteur::include::*;


pub struct TPrimRulesSection;


impl Section for TPrimRulesSection {

    fn new() -> Box<dyn Section> {
        Box::from(TPrimRulesSection)
    }

    fn new_token(&mut self, _token: Token) {}
    
}
