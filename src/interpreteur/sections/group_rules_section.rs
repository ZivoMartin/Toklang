use crate::interpreteur::include::*;


pub struct GroupRulesSection<'a> {
    text: &'a str
}


impl<'a> Section<'a> for GroupRulesSection<'a> {

    fn new(text: &'a str) -> Box<dyn Section<'a> + 'a> {
        Box::from(GroupRulesSection::<'a>{text})
    }

    fn new_token(&mut self, maps: &mut Maps, _token: Token) {}
    
}
