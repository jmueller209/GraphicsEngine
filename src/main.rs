use game::run; 

fn main() {
    // 2. Call the function
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
