mod node;
mod queue;
mod stack;
mod hash_map;

use hash_map::HashMap as HashMap;
use queue::Queue;
use stack::Stack;

fn main() {
    run_stack();
    run_queue();
    run_hash_map();
}

fn run_stack() {
    println!("--- Stack ---");
    let mut my_stack: Stack<i32> = Stack::new();
    println!("Stack empty? {}", if my_stack.is_empty() { "Yes" } else { "No" });
    my_stack.push(1);
    my_stack.push(3);
    println!("Stack length: {}", my_stack.length());
    println!("Stack top: {:?}", my_stack.peek());
    println!("Stack pop: {:?}", my_stack.pop());
    if let Some(item) = my_stack.peek() {
        println!("Last item: {}", item);
    } else {
        println!("Empty");
    }

    println!("Size: {}", my_stack.length());
    let is_empty = my_stack.is_empty();
    println!("Empty: {}\n", if is_empty { "Yes" } else { "No" });
}

fn run_queue() {
    println!("--- Queue ---");
    let mut my_queue: Queue<i32> = Queue::new();
    println!("Queue empty? {}", if my_queue.is_empty() { "Yes" } else { "No" });
    my_queue.push(10);
    my_queue.push(20);
    println!("Queue length: {}", my_queue.length());
    println!("Queue front: {:?}", my_queue.peek());
    println!("Queue pop: {:?}", my_queue.pop());
    println!("Queue front after pop: {:?}", my_queue.peek());
    println!("Queue empty now? {}\n", if my_queue.is_empty() { "Yes" } else { "No" });
}

fn run_hash_map() {
    println!("--- HashMap ---");
    let mut my_map: HashMap<&str, i32> = HashMap::new();
    println!("Map empty? {}", if my_map.is_empty() { "Yes" } else { "No" });

    my_map.insert("one", 1);
    my_map.insert("two", 2);
    println!("Map size: {}", my_map.size());
    println!("Get 'one': {:?}", my_map.get(&"one"));
    println!("Has key 'two'? {}", if my_map.has_key(&"two") { "Yes" } else { "No" });

    my_map.insert("one", 11);
    println!("Get updated 'one': {:?}", my_map.get(&"one"));

    my_map.remove("two");
    println!("Has key 'two' after remove? {}", if my_map.has_key(&"two") { "Yes" } else { "No" });
    println!("Map size now: {}\n", my_map.size());
}
