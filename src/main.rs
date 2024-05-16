use std::env;
mod tokenizer;
mod interpreteur;
use interpreteur::interpreteur::Interpreteur;
use std::process::exit;
use std::thread::spawn;
use tokenizer::{include::{TokenType, TokenizerMessage}, tokenizer::Tokenizer};
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::fs::{
    File,
    OpenOptions,
    create_dir
};
use std::io::Read;
use std::path::Path;

fn main() {
    let mut args = env::args().collect::<Vec<_>>();
    begin(args);
}

pub fn begin(mut args: Vec<String>) {
    let mut actions = Vec::<Box<dyn RequestParameter>>::new();
    let mut iter = args.iter_mut().skip(1);
    while let Some(elt) = iter.next() {
        match &elt as &str {
            "-f" => {
                let mut path = iter.next();
                if path.is_some() {
                    actions.push(OneFile::new(path.take().unwrap().to_string()))
                }else{
                    error_catched("You didn't precise the file path with the '-f' parameter.");
                }
            }
            _ => error_catched(&format!("Unknow parameter: {}", elt))
        }
    }
    let (sender, receiver) = channel::<TokenizerMessage>();
    let mut interp = Interpreteur::new();
    let mut tokenizer = Tokenizer::new(sender);
    for act in actions.iter_mut() {
        tokenizer = act.execute(tokenizer, &mut interp, &receiver);
    }
}


fn execute(interp: &mut Interpreteur, receiver: &Receiver<TokenizerMessage>) -> Result<Tokenizer, String> {
    let mut tokenizer: Option<Tokenizer> = None;
    while tokenizer.is_none() {
        match receiver.recv().expect("Something went wrong") {
            TokenizerMessage::Token(token) =>
                if token.token_type == TokenType::ERROR {
                    return Err(token.content)
                } else if let Err(e) = interp.new_token(token) {
                    return Err(e)
                }
            TokenizerMessage::Tokenizer(the_tokenizer) => tokenizer = Some(the_tokenizer)
        }
    }
    Ok(tokenizer.take().expect("Failed to catch the tokenizer throught the threads."))
}

trait RequestParameter {
    fn execute(&mut self, tokenizer: Tokenizer, interp: &mut Interpreteur, receiver: &Receiver<TokenizerMessage>) -> Tokenizer;    
}

struct OneFile {
    path: String
}

impl OneFile {
    fn new(path: String) -> Box<dyn RequestParameter> {
        Box::from(OneFile { path })
    }   
}

impl RequestParameter for OneFile {

    fn execute(&mut self, tokenizer: Tokenizer, interp: &mut Interpreteur, receiver: &Receiver<TokenizerMessage>) -> Tokenizer {
        let path = self.path.clone();
        spawn(move ||
              tokenizer.tokenize_file(path)
        );
        match execute(interp, receiver) {
            Ok(tokenizer) => {
                println!("The execution of the file {} has been a success.", self.path);
                return tokenizer;
            }
            Err(e) => error_catched(&e)
        };
        panic!("Impossible case");
    }
}


fn error_catched(err: &str) {
    println!("{err}");
    exit(1)
}
