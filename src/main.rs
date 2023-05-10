use std::{
    io::{self, Write},
    env,
    process,
    fs,
};


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => run_prompt(),
        1 => run_file(&args[0]),
        num_args => {
            eprintln!("Expected 1 argument but {} were given", num_args);
            process::exit(64);
        },
    };
}

fn run_file(path: &String) {
    let source = fs::read_to_string(path).unwrap();
    run(source);
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if let Err(error) = io::stdin().read_line(&mut line) {
            eprintln!("Error reading line: {:?}", error);
        }

        line = line.trim().to_string();
        
        run(line);
    }
}

fn run(source: String) {
    let scanner = Scanner{ source };
    let tokens = scanner.scan_tokens();
    
    for token in tokens {
        println!("{:?}", token);
    }
}

struct Scanner {
   source: String, 
}

impl Scanner {
    fn scan_tokens(&self) -> Vec<Token> {
        vec![Token{ s: "a".to_string() }]
    }
}
#[derive(Debug)]
struct Token {
    s: String,
}

















