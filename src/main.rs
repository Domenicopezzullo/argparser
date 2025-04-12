use argparser::ArgumentParser;

fn main() {
    let mut parser = ArgumentParser::default();
    parser.add_option("test", "no", "false");
    parser.add_option("te2", "yes", "false");
    let parsed = parser.parse().unwrap();
    println!("{}", parsed.get("test").unwrap());
    println!("{}", parsed.get("te2").unwrap());
}
