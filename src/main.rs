use argparser::{FlagType, Parser};

fn main() {
    let mut parser = Parser::new();
    parser.add_flag(
        "hi".to_string(),
        "no".to_string(),
        "no".to_string(),
        FlagType::STRING,
    );
    let a = parser.parse().unwrap();
    println!("{}", a.get("hi").unwrap());
}
