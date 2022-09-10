use hello::hello_world;
mod hello {
   pub fn hello_world() {
        println!("Hello, world!");
    }
}

fn main() {
    hello_world();
}
