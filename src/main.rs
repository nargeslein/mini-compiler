mod handwritten;
mod ai_generated; 

fn main() {
        let input = "let z = (x - 1) * 2 / y;";
    println!("handwritten:  {:?}", handwritten::lex(input));
    println!("ai_generated: {:?}", ai_generated::lex(input));
}