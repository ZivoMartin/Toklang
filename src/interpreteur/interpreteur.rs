use super::include::*;

pub struct Interpreteur {
//     request_treaters: Vec<Box<dyn Request>>,
//     keyword_link: HashMap<String, usize>,
//     current_treater: usize,
//     database: Database,
//     request_in_treatment: bool
}

impl Interpreteur {

    pub fn new() -> Interpreteur{
        Interpreteur {
            // request_treaters: Interpreteur::build_treaters(),
            // keyword_link: Interpreteur::build_keyword_link(),
            // current_treater: 0,
            // database: Database::load(),
            // request_in_treatment: false
        }
    }

    pub fn new_token(&mut self, _token: Token) -> ConsumeResult {
        Ok(())
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

