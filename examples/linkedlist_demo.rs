extern crate rust_collections;

use rust_collections::linkedlist::LinkedList;
// use std::collections::LinkedList;
fn main() {
	let mut list1 = LinkedList::new();
	list1.push_back('a');

	let mut list2 = LinkedList::new();
	list2.push_back('b');
	list2.push_back('c');

	list1.append(&mut list2);
	println!("linkedlist: {} with len=  {}", list1, list1.len());
	let mut iter = list1.iter();
	assert_eq!(iter.next(), Some(& 'a'));
	assert_eq!(iter.next(), Some(& 'b'));
	assert_eq!(iter.next(), Some(& 'c'));
	assert!(iter.next().is_none());

	assert!(list2.is_empty());

	let mut dl = LinkedList::new();

	dl.push_front(2);
	dl.push_front(1);
	assert_eq!(dl.len(), 2);
	assert_eq!(dl.front(), Some(&1));

	dl.clear();
	assert_eq!(dl.len(), 0);
	assert_eq!(dl.front(), None);

	let mut dl = LinkedList::new();
	assert_eq!(dl.back(), None);

	dl.push_back(1);
	assert_eq!(dl.back(), Some(&1));


	let mut d = LinkedList::new();
	assert_eq!(d.pop_front(), None);

	d.push_front(1);
	d.push_front(3);
	assert_eq!(d.pop_front(), Some(3));
	assert_eq!(d.pop_front(), Some(1));
	assert_eq!(d.pop_front(), None);

	let mut d = LinkedList::new();
	assert_eq!(d.pop_back(), None);
	d.push_back(1);
	d.push_back(3);
	assert_eq!(d.pop_back(), Some(3));
}