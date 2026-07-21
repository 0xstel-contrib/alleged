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
