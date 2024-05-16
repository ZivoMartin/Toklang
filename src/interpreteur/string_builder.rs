use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct StringBuilder {
    string: Option<String>
}

impl StringBuilder {

    pub fn new() -> StringBuilder {
        StringBuilder {
            string: Some(String::new())
        } 
    }

    pub fn from_string(s: String) -> StringBuilder {
        StringBuilder {
            string: Some(s)
        } 
    }

    fn string(&self) -> &String {
        self.string.as_ref().expect("StringBuilder: Failed to unwrap the string")
    }

    fn string_mut(&mut self) -> &mut String {
        self.string.as_mut().expect("StringBuilder: Failed to unwrap the string as mutable")
    }

    pub fn extract(&mut self) -> String {
        let res = self.string.take().expect("StringBuilder: Failed to unwrap the string during the extraction");
        self.string = Some(String::new());
        res
    }
    
    pub fn new_char(&mut self, c: String) {
        self.string_mut().push_str(&c);
    }

    pub fn hash(&self) -> i64 {
        let mut s = DefaultHasher::new();
        self.string().hash(&mut s);
        (s.finish()/2) as i64
    }

    pub fn is_empty(&self) -> bool {
        return self.string().is_empty()
    }
    
}
