mod args;

fn main() {
    let args = args::Args::load();
    println!("{:?}", args);

    println!("Hello, world!");
}
