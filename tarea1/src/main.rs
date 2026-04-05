mod node;
mod queue;
mod stack;

use queue::Queue;
use stack::Stack;

fn main() {

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
    println!("Empty: {}", if is_empty { "Yes" } else { "No" });

    println!("--- Queue ---");
    let mut my_queue: Queue<i32> = Queue::new();
    println!("Queue empty? {}", if my_queue.is_empty() { "Yes" } else { "No" });
    my_queue.push(10);
    my_queue.push(20);
    println!("Queue length: {}", my_queue.length());
    println!("Queue front: {:?}", my_queue.peek());
    println!("Queue pop: {:?}", my_queue.pop());
    println!("Queue front after pop: {:?}", my_queue.peek());
    println!("Queue empty now? {}", if my_queue.is_empty() { "Yes" } else { "No" });
}
