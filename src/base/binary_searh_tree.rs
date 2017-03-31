#![allow(non_snake_case)]

use std::fmt::Debug;

type TreeNode<K, V> = Option<Box<Node<K, V>>>;
#[derive(Debug)]
struct Node<K: PartialOrd + Debug, V: Debug> {
    key: K,
    value: V,
    left: TreeNode<K, V>,
    right: TreeNode<K, V>
}

trait BinaryTree<K, V> {
    fn pre_order(&self);
    fn in_order(&self);
    fn post_order(&self); 
}

trait BinarySearchTree<K: PartialOrd + Debug, V: Debug> : BinaryTree<K, V> {
    fn insert(&mut self, key: K, value: V);
}

impl<K: PartialOrd + Debug, V: Debug> BinaryTree<K, V> for Node<K, V> {
    
    fn pre_order(&self) {
        
        println!("key: {:?}, value: {:?}", self.key, self.value);

        if let Some(ref left) = self.left {
            
            left.pre_order();
        }

        if let Some(ref right) = self.right {

            right.pre_order();
        }
    }

    fn in_order(&self) {

        if let Some(ref left) = self.left {

            left.in_order();
        }

        println!("key: {:?}, value: {:?}", self.key, self.value);

        if let Some(ref right) = self.right {

            right.in_order();
        }
    }

    fn post_order(&self) {

        if let Some(ref left) = self.left {

            left.post_order();
        }

        if let Some(ref right) = self.right {

            right.post_order();
        }

         println!("key: {:?}, value: {:?}", self.key, self.value);
    }
}

impl<K: PartialOrd + Debug, V: Debug> BinarySearchTree<K, V> for Node<K, V> {

    fn insert(&mut self, key: K, value: V) {

        if self.key < key {
            
            if let Some(ref mut right) = self.right {

                right.insert(key, value);
            } else {

                self.right = Some(Box::new(Node::new(key, value)));
            }
        } else {

            if let Some(ref mut left) = self.left {

                left.insert(key, value);
            } else {

                self.left = Some(Box::new(Node::new(key, value)));
            }
        }
    }
}

impl<K: PartialOrd + Debug, V: Debug> Node<K, V> {
    
    fn new(key: K, value: V) -> Self {
        Node {
            key: key,
            value: value,
            left: None,
            right: None
        }
    }
}

type BST<K, V> = Node<K, V>;

pub fn test() {

    let mut root = BST::<i32,i32>::new(3,4);

    root.insert(2,3);
    root.insert(4,6);
    root.insert(5,5);
    root.insert(6,6);
    root.insert(1,8);

    if let Some(ref left) = root.left {
        assert_eq!(left.value, 3);
    }

    if let Some(ref right) = root.right {

        assert_eq!(right.value, 6);

        if let Some(ref right) = right.right {
            assert_eq!(right.value, 5);
        }
    }

    println!("");
    println!("pre order traversal:");
    root.pre_order();

    println!("");
    println!("in order traversal:");
    root.in_order();

    println!("");
    println!("post order traversal:");
    root.post_order();
}