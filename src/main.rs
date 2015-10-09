use std::env::args;

fn main() {
    let who = args().skip(1);

    match who.len() {
        0 => println!("Hello, world!"),
        _ => {
            for name in who {
                println!("Hello, {}!", name);
            }
        }
    }
}
