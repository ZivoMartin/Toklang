use crate::interpreteur::include::*;


pub struct DefineSection;


impl Section for DefineSection {

    fn new() -> Box<dyn Section> {
        Box::from(DefineSection)
    }

    fn new_token(&mut self, _token: Token) {}
    
}
