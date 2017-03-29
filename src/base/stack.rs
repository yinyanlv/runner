#![allow(non_snake_case)]

#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>
}

#[derive(Debug)]
struct StackNode<T> {
    value: T,
    next: Option<Box<StackNode<T>>>
}

impl<T> Stack<T> {

    fn new() -> Stack<T> {

        Stack {
            top: None
        }
    }

    fn push(&mut self, val: T) {
        
        let mut newNode = StackNode::new(val);

        newNode.next = self.top.take();
        self.top = Some(Box::new(newNode));
    }

    fn pop(&mut self) -> Option<T> {
        
        let curTopNode = self.top.take();

        match curTopNode {
            None => None,
            Some(mut node) => {
                
                self.top = node.next.take();

                Some(node.value)
            }
        }
    }
}

impl<T> StackNode<T> {

    fn new(val: T) -> StackNode<T> {
        
        StackNode {
            value: val,
            next: None
        }
    }
}

pub fn test() {

    #[derive(PartialEq, Eq, Debug)]
    struct Test {
        value: i32
    }

    let a = Test {value: 1};
    let b = Test {value: 2};

    let mut stack = Stack::<&Test>::new();

    assert_eq!(stack.pop(), None);
   
    stack.push(&a);
    stack.push(&b);

    println!("{:?}", stack);

    assert_eq!(stack.pop(), Some(&b));

    println!("{:?}", stack);

    assert_eq!(stack.pop(), Some(&a));

    println!("{:?}", stack);

    assert_eq!(stack.pop(), None);

    println!("{:?}", stack);

    assert_eq!(stack.pop(), None);
}
