#![allow(non_snake_case)]

#[derive(Debug)]
struct Node {
    id: usize,
    name: String
}

#[derive(Debug, Clone)]
struct Edge {
    edge: bool
}

#[derive(Debug)]
struct Graph {
    width: usize,
    graph: Vec<Vec<Edge>>
}

impl Node {

    fn new(id: usize, name: String) -> Self {

        Node {
            id: id,
            name: name
        }
    }
}

impl Edge {

    fn new() -> Self {
        Edge {
            edge: false
        }
    }

    fn have_edge() -> Self {

        Edge {
            edge: true
        }
    }
}

impl Graph {

    fn new(width: usize) -> Self {

        Graph {
            width: width,
            graph: vec![vec![Edge::new();width];width]
        }
    }

    fn insert_edge(&mut self, n1: Node, n2: Node) {

        match n1.id < self.width && n2.id < self.width {

            true => {  // 无向图

                self.graph[n1.id][n2.id] = Edge::have_edge();
                self.graph[n2.id][n1.id] = Edge::have_edge();  
            },

            false => {

                panic!("your node id is bigger than width!");
            }
        }
    }
}

pub fn test() {

    let mut graph = Graph::new(2);
    
    let n1 = Node::new(0, "n1".to_string());
    let n2 = Node::new(1, "n2".to_string());
    
    graph.insert_edge(n1, n2);

    println!("{:?}", graph);
}