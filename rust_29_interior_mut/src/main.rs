use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Part 1: std::cell::Cell -> provide direct access using set() & get()
    // https://doc.rust-lang.org/std/cell/
    // Notes:
    // 1- May be mutated through share reference
    // 2- Cell<T> requires T: Copy
    // 3- Cannot be used in a multithreading context (e.g. does not implement Sync)

    let cell = Cell::new(0);
    let value = cell.get();
    println!("[original value] cell value: {}", value);

    let new_value = cell.get() + 1;
    // Note: foo is going to mutate cell (by reference !!)
    foo(&cell);
    println!("[after foo] cell value: {}", cell.get());

    // Note: As this line erase the work done by the function 'foo', we must be careful
    //       when using Cell
    cell.set(new_value);
    println!("[after set] cell value: {}", cell.get());

    // Part 2: using Cell, we could implement a naive Rc like structure
    // (see std::rc::Rc documentation https://doc.rust-lang.org/std/rc/struct.Rc.html)

    let w = NaiveRc::new(String::from("Foo"));
    assert_eq!(w.ref_count(), 1);
    let w2 = w.clone();
    assert_eq!(w.ref_count(), 2);
    assert_eq!(w2.ref_count(), 2);
    drop(w2);
    // assert_eq!(w.ref_count(), 1); // Drop has not been impl for NaiveRc

    // Part 3: Implementing a Graph structure using RefCell
    let node_1 = Node::new(1);
    let node_2 = Node::new(2);
    let node_3 = Node::new(3);

    node_1.add_adjacent(&node_2);
    node_1.add_adjacent(&node_3);
    node_2.add_adjacent(&node_1);
    node_3.add_adjacent(&node_1);

    let nodes = vec![node_1, node_2, node_3];
    let g = Graph::from_nodes(nodes);

    // Show every nodes in the graph
    for node in g.nodes.iter().map(|node| node.0.borrow()) {
        let value = node.inner;
        let neighbours = node
            .adjacent
            .iter()
            .map(|node| node.borrow().inner)
            .collect::<Vec<_>>();
        println!("Node (value: {}), is connected to: {:?}", value, neighbours);
    }
}

/// a dummy function updating a Cell<i32>
fn foo(cell: &Cell<i32>) {
    let value = cell.get();
    cell.set(value + 2);
}

/// A naive clone of stc::rc::Rc
struct NaiveRc<T> {
    inner: T,
    refs: Cell<usize>,
}

impl<T> NaiveRc<T>
where
    T: Clone,
{
    fn new(inner: T) -> Self {
        Self {
            inner,
            refs: Cell::new(1),
        }
    }

    fn ref_count(&self) -> usize {
        self.refs.get()
    }
}

impl<T> Clone for NaiveRc<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        self.refs.set(self.refs.get() + 1);
        Self {
            inner: self.inner.clone(),
            refs: self.refs.clone(),
        }
    }
}

// End of Rc clone

// Graph

/// Represents a reference to a node.
/// This makes the code less repetitive to write and easier to read.
type NodeRef<T> = Rc<RefCell<_Node<T>>>;

/// The private representation of a node.
struct _Node<T> {
    inner: T,
    adjacent: Vec<NodeRef<T>>,
}

/// The public representation of a node, with some syntactic sugar.
struct Node<T>(NodeRef<T>);

impl<T> Node<T> {
    /// Create a new node with no edges
    fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(_Node {
            inner: value,
            adjacent: vec![],
        })))
    }

    /// Adds a directed edge from this node to other node.
    fn add_adjacent(&self, other: &Node<T>) {
        self.0
            .borrow_mut() // RefMut<_Node<T>>
            .adjacent
            .push(other.0.clone())
    }
}

struct Graph<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Graph<T> {
    fn from_nodes(nodes: Vec<Node<T>>) -> Self {
        Self { nodes }
    }
}

// End Graph
