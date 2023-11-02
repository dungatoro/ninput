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

#[macro_export]
macro_rules! input {
    ($($arg:tt)*) => {{
        let msg = std::fmt::format(std::format_args!($($arg)*));
        ninput(msg.as_str())
    }}
}
