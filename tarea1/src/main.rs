mod node;
mod queue;
mod stack;

use queue::Queue;
use stack::Stack;

fn main() {
    println!("Hello, world!");
    let mut my_stack: Stack<i32> = Stack::new();
    my_stack.push(1);
    my_stack.push(3);
    my_stack.pop();
    let last_item: Option<&i32> = my_stack.peek();
    if let Some(item) = last_item {
        println!("Last item: {}", item);
    } else {
        println!("Empty");
    }

    println!("Size: {}", my_stack.length());
    let is_empty = my_stack.is_empty();
    println!("Empty: {}", if is_empty { "Yes" } else { "No" });
}
