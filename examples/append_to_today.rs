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
