use std::thread;

fn main() {

    for i in 1..10 {

        println!("current i is: {}", i);

        let temp1 = thread::spawn(move || {

            println!("spawn thread {}", i);
        });

        println!("thread begin {}", i);

        let temp2 = thread::Builder::new()
                    .name("thread".to_string())
                    .stack_size(1024 * 1024 * 5)
                    .spawn(move || {

                        println!("builder thread {}", i);
                    });

        temp1.join().unwrap();

        println!("thread process {}", i);
       
        temp2.unwrap().join().unwrap();

        println!("thread end {}", i);
    }

    println!("main end");
}