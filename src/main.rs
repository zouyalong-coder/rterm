use std::io::Write;

mod lexer;

const PROMPT: &str = ">> ";


fn main() {
    loop {
        print!("{}", PROMPT);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut lexer = lexer::Lexer::new(&input);
        let cmd = lexer.next().unwrap();
        let exe = get_excutable(cmd.chars);
        if let Some(exe) = exe {
            let args = lexer.map(|token| token.chars.to_string()).collect();
            exe.run(args);
        } else {
            println!("command not found: {}", cmd.chars);
        }
    }
}

struct Executable{
    path: String,
}

impl Executable {
    fn run(&self, args: Vec<String>) {
        let mut cmd = std::process::Command::new(&self.path);
        cmd.args(args);
        let output = cmd.output().unwrap();
        println!("output: \n\t{}", String::from_utf8(output.stdout).unwrap());
    }
}

fn get_excutable(cmd: &str) -> Option<Executable> {
    let path = std::env::current_dir().unwrap();
    let path = path.join(cmd);
    if path.exists() {
        Some(Executable { path: path.to_str().unwrap().to_string() })
    } else {
        None
    }
}
