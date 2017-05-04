fn main() {

    let mut a = Box::new("abc");

    println!("main before abc run a is: {}", a);

    let b = abc(&mut a);

    println!("main after abc run a is {}", a);
   
    println!("main after abc run b is {}", b);
}

fn abc(a: &mut Box<&str>) -> &'static str {

    let temp: &str = get();

    *a = Box::new("abc changed");

    print_temp(temp);
  
    temp
}

fn get() -> &'static str {

    "xyz"
}

fn print_temp(a: &str) {
 
    println!("temp is: {}", a);
}