use std::io::Write;

pub fn ninput(message: &str) -> String {
    let mut input: String;
    print!("{message}");
    std::io::stdout().flush().expect("Failed to flush stdout");
    input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.pop();
    input
}
