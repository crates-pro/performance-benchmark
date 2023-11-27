use std::{thread::sleep, time::Duration};

fn main() {
    let mut x = 0;
	let y = 2;
	let z = 3;
	x = y + z;

	use std::env::args_os;
	let arg_os = args_os();
	let i = arg_os.count();
	sleep(Duration::from_secs(i.try_into().unwrap()));
}

#[test]
fn mytest_1() {
    let mut x = 0;
	let y = 2;
	let z = 3;
	x = y + z;
}

#[test]
fn mytest_2() {
    let mut x = 0;
	let y = 2;
	let z = 3;
	x = y + z;
}

