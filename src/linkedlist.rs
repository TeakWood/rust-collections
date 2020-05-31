use std::ptr::NonNull;
use std::mem;
use std::iter::{Iterator};
use std::fmt::{Display, Formatter, Error};

struct Node<T> {
	val: T,
	prev: Option<NonNull<Node<T>>>,
	next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
	pub fn new(val: T) -> Self {
		Node {
			val,
			prev: None,
			next: None,
		}
	}

	pub fn into_val(self: Box<Self>) -> T {
		self.val
	}
}

pub struct LinkedList<T> {
	head: Option<NonNull<Node<T>>>,
	tail: Option<NonNull<Node<T>>>,
	len: usize,
}


impl<T> LinkedList<T> 
where
	T: Display,
{
	pub fn new() -> Self {
		LinkedList {
			head: None,
			tail: None,
			len: 0,
		}
	}

	// For some reason directly using NonNull::new(&mut Node::new(val)) doesnt work
	// This has to be replaced with how standard collections are used by Boxing
	pub fn push_back(&mut self, val: T) {
		let mut node = Box::new(Node::new(val));
		node.next = None;
		node.prev = self.tail;
		let node = Some(Box::into_raw_non_null(node));
		// let node = Some(NonNull::new(Box::leak(node)));
		
		unsafe {
			match self.tail {
				Some(tail) => {
					(*tail.as_ptr()).next = node;
				}
				None => {
					self.head = node;
				},
			}
			self.tail = node;
		}
		
		self.len += 1;
	}

	pub fn append(&mut self, other: &mut LinkedList<T>) {
		// println!("Merging other with {} ele into self with {} ele", other.len(), self.len());
		match self.tail {
			Some(mut tail) => {
				if let Some(mut other_head) = other.head.take() {
					unsafe {
						tail.as_mut().next = Some(other_head);
						other_head.as_mut().prev = Some(tail);
					}
					self.tail = other.tail.take();
				}

			}
			None => mem::swap(self, other),
		}
		self.len += other.len;
	}

	pub fn is_empty(&self) -> bool {
		match self.head {
			Some(_) => false,
			None => true,
		}
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn iter(&self) -> LinkedListIter<'_, T> {
		LinkedListIter {
			list: &self,
			ptr: self.head,
		}
	}

	pub fn iter_mut(&self) -> LinkedListIterMut<'_, T> {
		LinkedListIterMut {
			list: &self,
			ptr: self.head,
		}
	}

	pub fn front(&self) -> Option<&T> {
		if None == self.head {
			return None;
		}
		unsafe {
			Some(&(&*self.head.unwrap().as_ptr()).val)
		}
	}

	pub fn push_front(&mut self, elt: T) {
		let mut node = Box::new(Node::new(elt));
		node.prev = None;
		node.next = self.head;
		let node = Some(Box::into_raw_non_null(node));
		if None == self.head {
			self.tail = node;
		}
		self.head = node;
		self.len += 1;
	}

	pub fn pop_front(&mut self) -> Option<T> {
		 self.head.map(|node| unsafe {
			let mut node = Box::from_raw(node.as_ptr());
			self.head = node.next;
			node.next = None;
			node.into_val()
		 })
	}

	pub fn back(&self) -> Option<&T> {
		if None == self.tail {
			return None;
		}
		unsafe {
			Some(&(&*self.tail.unwrap().as_ptr()).val)
		}
	}

	pub fn pop_back(&mut self) -> Option<T> {
		self.tail.map(|node| unsafe {
			let mut node = Box::from_raw(node.as_ptr());
			self.tail = node.prev;
			node.prev = None;
			node.into_val()
		 })
	}

	pub fn clear(&mut self) {
		let mut ptr = self.head;
		loop {
			match ptr {
				Some(current) => unsafe {
					let node = &*current.as_ptr();
					ptr = node.next;
					drop(node);
					self.len -= 1;
				},
				None => break,
			}
		}
		self.head = None;
		self.tail = None;
	}
}

pub struct LinkedListIter<'a, T> {
	list: &'a LinkedList<T>,
	ptr: Option<NonNull<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> 
where
	T: Display
{
	type Item = &'a T;
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.list.len == 0 {
			println!("Reached end");
			return None;
		}

		match self.ptr {
			Some(current) => unsafe {
				let node = &*current.as_ptr();
				self.ptr = node.next;
				Some(&node.val)
			},
			None => None,
		}
	}
}

pub struct LinkedListIterMut<'a, T> {
	list: &'a LinkedList<T>,
	ptr: Option<NonNull<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
	type Item = &'a T;
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.list.len == 0 {
			return None;
		}
		self.ptr = self.list.head;
		match self.ptr {
			Some(current) => unsafe {
				let node = &mut *current.as_ptr();
				self.ptr = node.next;
				// println!("Ele = {}", &node.val);
				Some(&mut node.val)
			},
			None => None,
		}
	}
}


impl<T> Display for LinkedList<T>
where
	T: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		write!(f, "[");
		let mut ptr = self.head;
		match ptr {
			Some(node) => unsafe {
				write!(f, "{}", (*node.as_ptr()).val);
				ptr = (&*node.as_ptr()).next;
				loop {
					match ptr {
						Some(node) => {
							write!(f, "-->{}", (*node.as_ptr()).val);
							ptr = (&*node.as_ptr()).next;
						},
						None => break write!(f, "]"),
					}
				}
			},
			None => {
				write!(f, "]")
			},
		}
	}
}