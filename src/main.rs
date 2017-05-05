fn main() {

    let mut a = "abc";

    println!("main before abc run a is: {}", a);
    
    let b = abc(&mut a);

    println!("main after abc run a is {}", a);
    println!("main after abc run b is {}", b);
}

fn abc<'a>(a: &'a mut &str) -> &'static str {

    let temp: &str = get();

    *a = "abc changed";

    print_temp(temp);
  
    temp
}

fn get() -> &'static str {

    "xyz"
}

fn print_temp(a: &str) {
 
    println!("temp is: {}", a);
}