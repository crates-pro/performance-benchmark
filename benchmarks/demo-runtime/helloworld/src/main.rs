use std::env::args_os;

fn main() {
    let arg_os = args_os();
    println!("{:?}", arg_os);
}
