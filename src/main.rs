// LinkedList Implementation

struct Node{ // declares the node struct
    dataset:i32,   // holds data with type of i32 
    next: Option<Box<Node>>, // an option that points to the next node or none
}

// a constructor for the node
impl Node {
    fn new(dataset:i32)->Self{  // a constructor function that create the new node.. next node to none
        Node{
            dataset,
            next:None,
        }
    }
}

// defining linkedlist struct 
struct LinkedList{
    head:Option<Box<Node>>,
}

// constructor fn for implementing the linkedlist
impl LinkedList {
    fn new()->Self{
        LinkedList{head:None}
    }
}

// constructor fn for push_back
impl LinkedList{
    fn push_back(&mut self, dataset:i32){
        let new_node = Box::new(Node::new(dataset));
        match self.head{
            None=>{
                self.head = Some(new_node);
            }
            Some(ref mut head_node)=>{
                let mut tail =head_node;
                while let Some(ref mut next_node) =tail.next  {
                    tail = next_node;
                }
                tail.next = Some(new_node);
            }
        }
    }
}

// adding a new node to the front
impl LinkedList {
    fn push_front(&mut self, dataset:i32){
        let mut new_node = Box::new(Node::new(dataset));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }
}

// traversing the list
impl LinkedList{
    fn traversing(&self){
        let mut current = &self.head;
        while let Some(node) =current{
            println!("{}",node.dataset);
            current = &node.next;
        }
    }
}

// printing the list
impl LinkedList{
    fn output_list(&self){
        let mut current = &self.head;
        while let Some(node)=current{
            println!("{}",node.dataset);
            current = &node.next;       
        }
    }
}

// removing a node from the front
impl LinkedList {
    fn pop_front(&mut self)->Option<i32>{ // removes the front node and returns its data
        self.head.take().map(|node| {self.head=node.next;
            node.dataset

        })
    }
}

// testing the linkedlist

fn main(){
    let mut list = LinkedList::new();

    for i in 1..7 {
        list.push_front(i);
    }
    //popping from the list
    println!("Popping from the list");
    if let Some(dataset)=list.pop_front(){
        println!("popped: {}",dataset);
    }
    //traversing the link
    println!("Traversing the link");
    list.traversing();

    // outputing the entire list

    println!("outputing the entire list");
    list.output_list();

    // pushing dataitems to the back
    println!("pushing dataitems to the back");
    list.push_back(3);

    println!("after pushing");
    println!("outputing the entire list");
    list.output_list();

}