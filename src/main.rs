mod token;

fn main() {
    let tokens = token::parse("if x > 10 { return x } else { return 0 }");
    println!("Parsed tokens: {:?}", tokens);
}
