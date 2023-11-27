fn main() {
    println!("hello world");
    let second_2 = std::time::Duration::from_millis(1000);
    std::thread::sleep(second_2);
}