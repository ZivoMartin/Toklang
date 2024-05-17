use crate::interpreteur::include::*;


pub struct GroupRulesSection;


impl Section for GroupRulesSection {

    fn new() -> Box<dyn Section> {
        Box::from(GroupRulesSection)
    }

    fn new_token(&mut self, _token: Token) {}
    
}
