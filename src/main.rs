use game::run; 

fn main() {
    // 2. Call the function
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

// Testing
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn new(x: T, y: T) -> Self {
//         Point { x, y }
//     }
// }
//
// fn main() {
//     let x : f64 = 3.14;
//
// }

