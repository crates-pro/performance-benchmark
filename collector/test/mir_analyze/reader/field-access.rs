struct A {
    field: String,
}

fn main() {
    let a = A { field: String::from("string") };

    println!("{}", a.field);
    
    let mut b = A { field: a.field };

    b.field.clear();
}