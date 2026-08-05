use alleged_lib::graph::Graph;
use std::fs;

fn main() {
    let notes = Graph::builder()
        .root("assets/example_graph".into())
        .build()
        .unwrap();

    let mut today = notes.today().unwrap();
    today.append_block("Hello from Rust code!", 0).unwrap();

    notes.save(&mut today).unwrap();

    let today_content = fs::read_to_string(today.path()).unwrap();
    assert!(today_content.contains("Hello from Rust code!"));
}
