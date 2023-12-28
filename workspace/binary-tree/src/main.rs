use std::{cell::RefCell, rc::Rc};

/// Define node structure
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

/// Define binary tree structure
#[derive(Debug)]
struct BinaryTree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    /// constructor for creating a new node
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T: std::fmt::Debug> BinaryTree<T> {
    /// constructor for creating a new binary tree
    fn new() -> Self {
        BinaryTree { root: None }
    }

    /// Insert a value into a binary tree
    fn insert(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        if let Some(root) = &self.root {
            self.insert_recursive(root.clone(), new_node.clone());
        } else {
            self.root = Some(new_node)
        }
    }

    /// Helper function for recursive insert
    fn insert_recursive(&self, current: Rc<RefCell<Node<T>>>, new_node: Rc<RefCell<Node<T>>>) {
        let mut current = current.borrow_mut();

        // insert left node is available, insert child here
        if current.left.is_none() {
            current.left = Some(new_node)
        }
        // or else insert at right child
        else if current.right.is_none() {
            current.right = Some(new_node)
        }
        // if both child are full, recursively insert into the left child
        else if let Some(left) = &current.left {
            self.insert_recursive(left.clone(), new_node.clone());
        }
    }

    /// in order traversal of binary tree
    fn inorder_traversal(&self, node: &Option<Rc<RefCell<Node<T>>>>) {
        if let Some(current) = node {
            let current = current.borrow();
            self.inorder_traversal(&current.left);
            print!("{:?}", current.value);
            self.inorder_traversal(&current.right);
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(1);
    tree.insert(2);
    tree.insert(3);
    tree.insert(4);
    tree.insert(5);
    tree.inorder_traversal(&tree.root); // prints 42513
}
