use std::env;
mod tokenizer;
mod interpreteur;
use interpreteur::interpreteur::Interpreteur;
use std::process::exit;
use std::thread::spawn;
use tokenizer::{include::{TokenType, TokenizerMessage}, tokenizer::Tokenizer};
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::fs::File;
use std::io::Read;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("File path missing");
        exit(1);
    }
    begin(&args[1]);
}

pub fn begin(path: &str) {
    let mut file: File = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("The path {} isn't valid.", path);
            exit(1)
        }
    };
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read entry file");
    let (sender, receiver) = channel::<TokenizerMessage>();
    let mut interp = Interpreteur::new(&content);
    let tokenizer = Tokenizer::new(sender);
    let moved_path = path.to_string();
    spawn(move ||
          tokenizer.tokenize_file(moved_path)
    );
    match execute(&mut interp, &receiver) {
        Ok(_tokenizer) => println!("The execution of the file {} has been a success.", path),
        Err(e) => println!("Error: {e}")
    };
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
