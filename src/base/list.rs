use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil  // 一个链表节点的末端
}

impl List {
    
    fn new() -> Self {
        Nil
    }

    fn prepend(self, item: u32) -> List {

        Cons(item, Box::new(self))
    }

    fn len(&self) -> u32 {

        match *self {
            Cons(_, ref list) => 1 + list.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {

        match *self {
            Cons(head, ref list) => {
                format!("{}, {}", head, list.stringify());
            },
            Nil => {
                format!("Nil");
            }
        }
    }
}

pub fn test() {

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}