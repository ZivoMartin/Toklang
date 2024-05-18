use std::env;
mod tokenizer;
mod interpreteur;
use interpreteur::interpreteur::Interpreteur;
use std::process::exit;
use std::thread::spawn;
use tokenizer::{include::{TokenType, TokenizerMessage, PARSING_ERROR}, tokenizer::Tokenizer};
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
    let tokenizer = Tokenizer::new(content.clone(), sender);
    spawn(move ||
          tokenizer.tokenize_file()
    );
    match execute(&mut interp, &receiver) {
        Ok(_tokenizer) => println!("The execution of the file {} has been a success.", path),
        Err(e) => println!("Error: {e}")
    };
}


fn execute(interp: &mut Interpreteur, receiver: &Receiver<TokenizerMessage>) -> Result<(), String> {
    loop  {
        match receiver.recv().expect("Something went wrong") {
            TokenizerMessage::Token(token) =>
                if token.token_type == TokenType::ERROR {
                    return Err(PARSING_ERROR.to_string())
                } else if let Err(e) = interp.new_token(token) {
                    return Err(e)
                }
            TokenizerMessage::End() => break
        }
    }
    Ok(())
}
