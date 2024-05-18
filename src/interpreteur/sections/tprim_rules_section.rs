use crate::interpreteur::include::*;


pub struct TPrimRulesSection<'a> {
    text: &'a str
}


impl<'a> Section<'a> for TPrimRulesSection<'a> {

    fn new(text: &'a str) -> Box<dyn Section<'a> + 'a> {
        Box::from(TPrimRulesSection {text})
    }

    fn new_token(&mut self, maps: &mut Maps, _token: Token) {}
    
}
