macro_rules! gen_func {

    ($func: ident) => {

        fn $func() {

            println!("current function is: {}", stringify!($func));
        }
    }

}

fn main() {

    gen_func!(abc);

    abc();
}