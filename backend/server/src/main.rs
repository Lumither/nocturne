mod constants;
mod scheduler;

const BUILD_ID: &str = env!("BUILD_ID");

fn main() {
    println!("{}", BUILD_ID);
}
