use argparser::ArgumentParser;

fn main() {
    let mut parser = ArgumentParser::default();
    parser.add_option("test", "no", "false");

    let parsed = parser.parse().unwrap();
    println!("{}", parsed["test"]);
}
