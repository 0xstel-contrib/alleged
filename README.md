# alleged-lib
Simple library to read/write to Logseq note graphs in Rust, built with [comrak](https://lib.rs/crates/comrak).

## Usage
Append to today's journal entry ([`examples/append_to_today.rs`](./examples/append_to_today.rs)):
```rust
use alleged_lib::graph::Graph;

fn main() {
    let notes = Graph::builder()
        .root("assets/my_graph".into())
        .build()
        .unwrap();

    let mut today = notes.today().unwrap();
    today.append_block("Hello from Rust code!", 0).unwrap();

    notes.save(&mut today).unwrap();
}
```

Mark all tasks in today's journal entry as done ([`examples/mark_tasks_done.rs`](./examples/mark_tasks_done.rs)):
```rust
use alleged_lib::{
    block::{Block, TaskMarker},
    ext::comrak::Arena,
    graph::{Document, Graph},
};

fn main() {
    let notes = Graph::builder()
        .root("assets/my_graph".into())
        .build()
        .unwrap();

    let mut today = notes.today().unwrap();

    let arena = Arena::new();
    let Document(root, blocks) = today.blocks(&arena);
    for mut block in blocks {
        if let Block::Task(ref mut task, _depth) = block {
            task.mark(TaskMarker::Done);
        }
    }

    today.update_buffer(root).unwrap();
    notes.save(&mut today).unwrap();
}
```


## Crate Features
- `regex-lite`: use the [regex-lite](https://lib.rs/crates/regex-lite) crate as the regular expression backend (**enabled by default**)
- `regex`: use the [regex](https://lib.rs/crates/regex) crate as the regular expression backend
- `serde`: enable support for [serde](https://serde.rs) serialization & deserialization of certain types
