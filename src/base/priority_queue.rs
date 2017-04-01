#![allow(non_snake_case)]

#[derive(Debug)]
struct PriorityQueue<T> where T: PartialOrd + Clone {
    data: Vec<T>
}


impl<T> PriorityQueue<T> where T: PartialOrd + Clone {
    
    fn new() -> Self {
        PriorityQueue {
            data: Vec::new()
        }
    }

    fn len(&self) -> usize {

        self.data.len()
    }

    fn is_empty(&self) -> bool {

        self.data.len() == 0
    }

    fn insert(&mut self, value: T) {

        self.data.push(value);
    }

    fn max(&mut self) -> Option<T> {
        
        if self.is_empty() {
            return None
        }

        let maxIndex = self.max_index();

        Some(self.data[maxIndex].clone())
    }

    fn delete_max(&mut self) -> Option<T> {

        if self.is_empty() {
            return None
        }

        let maxIndex = self.max_index();

        Some(self.data.remove(maxIndex).clone())
    }

    fn max_index(&self) -> usize {

        let mut maxIndex = 0;

        for i in 1..self.len() - 1 {
            if self.data[maxIndex] < self.data[i] {
                maxIndex = i;
            }
        }

        maxIndex
    }

    fn min(&mut self) -> Option<T> {
        
        if self.is_empty() {
            return None
        }

        let minIndex = self.min_index();

        Some(self.data[minIndex].clone())
    }
    
    fn delete_min(&mut self) -> Option<T> {

        if self.is_empty() {
            return None
        }

        let minIndex = self.min_index();

        Some(self.data.remove(minIndex).clone())
    }

    fn min_index(&self) -> usize {

        let mut minIndex = 0;

        for i in 1..self.len() - 1 {
            if self.data[minIndex] > self.data[i] {
                minIndex = i;
            }
        }

        minIndex
    }
}

fn test_min() {

    let mut pq = PriorityQueue::new();

    pq.insert(3);
    pq.insert(2);
    pq.insert(1);
    pq.insert(4);

    assert!(pq.min().unwrap() == 1);
}

fn test_max() {

    let mut pq = PriorityQueue::new();

    pq.insert(2);
    pq.insert(4);
    pq.insert(1);
    pq.insert(3);

    assert!(pq.max().unwrap() == 4);
}

fn test_is_empty() {

    let mut pq = PriorityQueue::new();

    assert!(pq.is_empty());

    pq.insert(1);

    assert!(!pq.is_empty());
}

fn test_len() {

    let mut pq = PriorityQueue::new();

    assert!(pq.len() == 0);

    pq.insert(2);
    pq.insert(4);
    pq.insert(1);

    assert!(pq.len() == 3);
}

fn test_delete_min() {

    let mut pq = PriorityQueue::new();

    pq.insert(3);
    pq.insert(2);
    pq.insert(1);
    pq.insert(4);

    assert!(pq.len() == 4);
    assert!(pq.delete_min().unwrap() == 1);
    assert!(pq.len() == 3);
}

fn test_delete_max() {

    let mut pq = PriorityQueue::new();

    pq.insert(2);
    pq.insert(10);
    pq.insert(1);
    pq.insert(6);
    pq.insert(3);

    assert!(pq.len() == 5);
    assert!(pq.delete_max().unwrap() == 10);
    assert!(pq.len() == 4);
}

pub fn test() {

    test_len();
    test_delete_max();
    test_delete_min();
    test_is_empty();
    test_max();
    test_min();
}