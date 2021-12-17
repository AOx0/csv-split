mod args;
mod file_manager;


fn main() {
    let args = args::Args::load();
    println!("{:?}", args);

    println!("Hello, world!");
}
