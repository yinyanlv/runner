#![allow(non_snake_case)]

#[derive(Debug)]
struct Queue<T> {
    data: Vec<Option<T>>
}

impl<T> Queue<T> {

    fn new() -> Queue<T> {
        Queue {
            data: Vec::new()
        }
    }

    fn enqueue(&mut self, val: T) {

        self.data.push(Some(val));
    }

    fn dequeue(&mut self) -> Option<T> {

        self.data.remove(0)
    }    
}

pub fn test() {

    #[derive(PartialEq, Eq, Debug)]
    struct Test {
        value: i8
    }

    let a = Test {value: 1};
    let b = Test {value: 2};
    let c = Test {value: 3};

    let mut queue = Queue::<Test>::new();

    println!("{:?}", queue);

    queue.enqueue(a);
    queue.enqueue(b);
    queue.enqueue(c);
    
    println!("{:?}", queue);

    queue.dequeue();
        
    println!("{:?}", queue);

    queue.dequeue();
        
    println!("{:?}", queue);

    queue.dequeue();
        
    println!("{:?}", queue);
}