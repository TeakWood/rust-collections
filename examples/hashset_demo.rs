extern crate rust_collections;

use rust_collections::hashset::HashSet;

pub fn main() {
	// Type inference lets us omit an explicit type signature (which
	// would be `HashSet<String>` in this example).
	let mut books = HashSet::new();

	// Add some books.
	books.insert("A Dance With Dragons".to_string());
	books.insert("To Kill a Mockingbird".to_string());
	books.insert("The Odyssey".to_string());
	books.insert("The Great Gatsby".to_string());

	// Check for a specific one.
	if !books.contains("The Winds of Winter") {
		println!("We have {} books, but The Winds of Winter ain't one.",
				books.len());
	}

	// Remove a book.
	books.remove("The Odyssey");

	// Iterate over everything.
	for book in &books {
		println!("{}", book);
	}

	#[derive(Hash, Eq, PartialEq, Debug)]
	struct Viking {
		name: String,
		power: usize,
	}

	let mut vikings = HashSet::new();

	vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
	vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
	vikings.insert(Viking { name: "Olaf".to_string(), power: 4 });
	vikings.insert(Viking { name: "Harald".to_string(), power: 8 });

	// Use derived implementation to print the vikings.
	for x in &vikings {
		println!("{:?}", x);
	}

	let viking_names: HashSet<&'static str> =
	[ "Einar", "Olaf", "Harald" ].iter().cloned().collect();
	for x in &viking_names {
		println!("{:?}", x);
	}
}