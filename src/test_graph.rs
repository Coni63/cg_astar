use std::cell::RefCell;
use std::rc::Rc;

// Define the Node struct
#[derive(Debug)]
struct Node {
    value: i32,
    up: Option<Rc<RefCell<Node>>>, // Use Rc and RefCell for mutability and reference counting
    down: Option<Rc<RefCell<Node>>>, // Use Rc and RefCell for mutability and reference counting
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            up: None,
            down: None,
        }))
    }

    fn add_neighbor(&mut self, direction: &str, neighbor: Rc<RefCell<Node>>) {
        match direction {
            "up" => self.up = Some(neighbor),
            "down" => self.down = Some(neighbor),
            _ => println!("Invalid direction"),
        }
    }

    fn show(&self) {
        println!("Value: {:?}", self.value);

        if let Some(up) = &self.up {
            println!("Up: {:?}", up.borrow().value);
        }
        if let Some(down) = &self.down {
            println!("Down: {:?}", down.borrow().value);
        }
    }
}

fn main() {
    // Create some nodes
    let node_a = Node::new(1);
    let node_b = Node::new(2);

    // Add references to each other
    node_a.borrow_mut().add_neighbor("up", Rc::clone(&node_b));
    node_a.borrow_mut().add_neighbor("down", Rc::clone(&node_b));
    node_b.borrow_mut().add_neighbor("down", Rc::clone(&node_a));

    node_a.borrow_mut().value = 3;

    // Print the nodes
    node_a.borrow().show();
    node_b.borrow().show();
}
