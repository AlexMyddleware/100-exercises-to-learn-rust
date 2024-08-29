// TODO: Re-implement `Ticket`'s accessor methods. This time return a `&str` rather than a `&String`.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &str {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};
    use std::any::{Any, TypeId};

    #[test]
    fn test_type() {
        let ticket = Ticket::new(valid_title(), valid_description(), "To-Do".to_string());
        // Some dark magic to verify that you used the expected return types
        assert_eq!(TypeId::of::<str>(), ticket.title().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.description().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.status().type_id());
    }
}

// World of Warcraft Example
// Imagine you have a big book called "World of Warcraft Lore". This book is like a String in Rust. It contains a lot of information, and you can think of it as a collection of characters stored in memory.

// String in Rust
// Book: The entire "World of Warcraft Lore" book.
// Pointer: A bookmark that tells you where the book starts.
// Length: The number of pages in the book.
// Capacity: The maximum number of pages the book can hold before you need a new book.

// When you create a String in Rust, it allocates memory to store the characters and keeps track of where the string starts, its length, and its capacity.

// let mut book = String::with_capacity(5);
// book.push_str("Hello");

// +---------+--------+----------+
// Stack | pointer | length | capacity | 
//       |  |      |   5    |    5     |
//       +--|------+--------+----------+
//          |
//          |
//          v
//        +---+---+---+---+---+
// Heap:  | H | e | l | l | o |
//        +---+---+---+---+---+


//        &String in Rust
//        Now, let's say you want to lend this book to a friend, but you don't want to give them the entire book, just a reference to it. This reference is like a &String in Rust.
//        Book Reference: A note that tells your friend where to find the book.
//        Pointer: The bookmark that points to the start of the book.
//        Length: The number of pages in the book.
//        Capacity: The maximum number of pages the book can hold.
//        In memory, it looks like this:

//        --------------------------------------
//        |                                    |         
//   +----v----+--------+----------+      +----|----+
//   | pointer | length | capacity |      | pointer |
//   |    |    |   5    |    5     |      |         |
//   +----|----+--------+----------+      +---------+
//        |        book                    &book 
//        |       
//        v       
//      +---+---+---+---+---+
//      | H | e | l | l | o |
//      +---+---+---+---+---+

// &str in Rust
// Now, let's say you want to give your friend just a part of the book, like a specific chapter or a few pages. This is like a &str in Rust, which is a slice of the original string.

// Book Slice: A note that tells your friend where to find a specific part of the book.
// Pointer: The bookmark that points to the start of the specific part.
// Length: The number of pages in that part.
// For example, if you want to give your friend the substring "ello" from "Hello":

// let part = &book[1..5];

// -------------------
// |                 |         
// +----v----+--------+   |
// | pointer | length |   |
// |    |    |   4    |   |
// +----|----+--------+   |
// |        part      |
// |       
// v       
// +---+---+---+---+
// | e | l | l | o |
// +---+---+---+---+

// Summary
// String: The entire book with its metadata (pointer, length, capacity).
// &String: A reference to the entire book.
// &str: A reference to a specific part of the book (substring).
// Using these concepts, you can efficiently manage and reference parts of your data in Rust, just like how you might manage and reference parts of a big book in real life.