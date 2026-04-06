//! Generic node used for Linked List

pub(crate) struct Node<T> {
	pub(crate) value: T,
	pub(crate) next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
	pub(crate) fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
		Self { value, next }
	}
}
