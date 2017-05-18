fn main() {
    let mut a = "abc";


    let n = change_a(&mut a);

    println!("n is: {}", n);
}

fn change_a<'a>(a: &'a mut &'a str) -> &'a str {

    println!("prev a is: {}", a);

    *a = "xyz";

    println!("changed a is: {}", a);

    "hh"
}