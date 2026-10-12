mod ai_generated;
mod handwritten;
#[cfg(test)]
mod shared_tests;
mod token;
fn main() {
    let input = "let z = (x - 1) * 2 / y;";
    println!("handwritten:  {:?}", handwritten::lex(input));
    println!("ai_generated: {:?}", ai_generated::lex(input));
}
