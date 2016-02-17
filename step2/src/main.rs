use std::env::args;

fn main() {
    let who = args().nth(1);

    match who {
        Some(name) => {
            println!("Hello, {}!", name);
        },
        None => {
            println!("Hello, world!");
        }
    }
}
