use std::fs;

fn main() {
    let content = fs::read_to_string("build/generated.txt").unwrap_or_else(|_| "MISSING".to_string());
    println!("generated: {}", content.trim());
}
