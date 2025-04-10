# todo-cli

A simple Todo List app built by following (freeCodeCamp)[https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/]. 

This project was intended to help me learn and practice the following concepts:
- Error handling in Rust
- Options and Null types
- Structs and impl
- Terminal I/O
- File system handling
- Ownership and borrow in Rust
- Match patterns
- Iterators and closures
- Using external crates

## Usage

1. `add` adds a todo item to the list in the form of "action item" (or marks an existing completed item as uncompleted).

`cargo run -- add "make coffee"`

2. `complete` marks an existing todo item as completed.

`cargo run -- complete "make coffee"`

Note: Todo lists are automatically saved into a `db.json` file and stored locally. `true` indicates the item is uncompleted, while `false` indicates the item is completed (i.e., it no longer exists in the list).

```
{
  "get sleep": true,
  "code rust": false
}
```
