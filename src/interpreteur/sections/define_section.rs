use crate::interpreteur::include::*;


pub struct DefineSection<'a> {
    text: &'a str
}


impl<'a> Section<'a> for DefineSection<'a> {

    fn new(text: &'a str) -> Box<dyn Section<'a> + 'a> {
        Box::from(DefineSection{text})
    }

    fn new_token(&mut self, maps: &mut Maps, _token: Token) {}
    
}
