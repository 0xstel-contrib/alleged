mod builder;
pub use builder::*;

use alleged_lib::{
    block::{Block, BlockImpl, TaskMarker},
    ext::comrak::Arena,
    graph::{Document, Graph},
    properties::Properties,
};
use anyhow::Result;
use http::Uri;
use icalendar::{Calendar, Component, Event, EventLike, Property, Todo, TodoStatus};
use std::str::FromStr;
use vstorage::{
    Etag,
    base::{CreateItemOptions, Item, Storage},
};

fn prepare_component<C, B>(mut base: C, block: &B) -> C
where
    C: Component + EventLike,
    B: BlockImpl,
{
    if let Some(due) = block.due() {
        base.starts(due);
    }

    base.summary(&block.plain())
        .append_property(Property::new("CONTENT-HASH", block.hash()));

    base
}

fn create_calendar_from(block: &Block<'_>, id: &str) -> Calendar {
    let mut calendar = Calendar::new();

    match block {
        Block::Text(text, _) => {
            calendar.push(prepare_component(Event::with_uid(id), text));
        }
        Block::Task(task, _) => {
            let task_status = match task.marker {
                TaskMarker::Cancelled => TodoStatus::Cancelled,
                TaskMarker::Doing => TodoStatus::InProcess,
                TaskMarker::Done => TodoStatus::Completed,
                TaskMarker::ToDo | TaskMarker::Waiting => TodoStatus::NeedsAction,
            };

            let mut component = prepare_component(Todo::with_uid(id), task);
            component.status(task_status);
            calendar.push(component);
        }
    }

    calendar.done()
}

fn content_hash<C: Component>(component: &C) -> Option<String> {
    component.property_value("CONTENT-HASH").map(String::from)
}

pub struct LogseqCaldav<S>
where
    S: Storage,
{
    graph: Graph,
    storage: S,
    collection: Uri,
}

impl<S> LogseqCaldav<S>
where
    S: Storage,
{
    #[must_use]
    pub fn builder() -> LogseqCaldavBuilder<S> {
        LogseqCaldavBuilder::default()
    }
    async fn fetch_stored_calendar_item(&self, href: &str) -> Option<(String, Item, Etag)> {
        if let Ok((stored_item, stored_etag)) = self.storage.get_item(href).await {
            let stored_calendar: Calendar = stored_item.as_str().parse().ok()?;
            if let Some(stored_hash) = stored_calendar.iter().find_map(|e| {
                e.as_event()
                    .map_or_else(|| e.as_todo().and_then(content_hash), content_hash)
            }) {
                return Some((stored_hash, stored_item, stored_etag));
            }
        }

        None
    }
    /// Syncs the status of tasks that exist both on the calendar and in Logseq, overwriting the Logseq task status with the calendar task status if they differ.
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub async fn caldav_statuses_to_graph(&self) -> Result<()> {
        for mut entry in self.graph.entries() {
            let arena = Arena::new();
            let Document(root, blocks) = entry.blocks(&arena);

            for mut block in blocks {
                let Properties(properties) = block.properties();

                if let Block::Task(ref mut task, _) = block {
                    // The graph is initialised with `populate_ids()`, so the ID field is guaranteed to be `Some`.
                    #[allow(clippy::unwrap_used)]
                    let id = properties.get("id").unwrap();
                    let href = format!("{}/{id}.ics", self.collection);

                    // The stored calendar item is guaranteed to be valid... I think?
                    // I know `vstorage`/`libdav` doesn't handle validation, but I'm pretty
                    // sure the string there is OK to be parsed into an `icalendar::Calendar`
                    #[allow(clippy::unwrap_used)]
                    if let Some((_, stored_item, _)) = self.fetch_stored_calendar_item(&href).await
                        && let Some(stored_todo) = Calendar::from_str(stored_item.as_str())
                            .unwrap()
                            .todos()
                            .next()
                    {
                        let new_status =
                            match stored_todo.get_status().unwrap_or(TodoStatus::NeedsAction) {
                                TodoStatus::Cancelled => TaskMarker::Cancelled,
                                TodoStatus::InProcess => TaskMarker::Doing,
                                TodoStatus::Completed => TaskMarker::Done,
                                TodoStatus::NeedsAction => TaskMarker::ToDo,
                            };
                        task.mark(new_status);
                    }
                }
            }

            entry.update_buffer(root)?;
            self.graph.save(&mut entry)?;
        }

        Ok(())
    }
    /// Writes all `due` items from the Logseq graph to the calendar server.
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub async fn graph_items_to_caldav(&self) -> Result<()> {
        for mut entry in self.graph.entries() {
            let arena = Arena::new();
            let Document(_, blocks) = entry.blocks(&arena);

            for block in blocks {
                if block.due().is_some() {
                    let hash = block.hash();
                    let Properties(properties) = block.properties();

                    // The graph is initialised with `populate_ids()`, so the ID field is guaranteed to be `Some`.
                    #[allow(clippy::unwrap_used)]
                    let id = properties.get("id").unwrap();
                    let href = format!("{}/{id}.ics", self.collection);
                    let calendar_data = create_calendar_from(&block, id).to_string();

                    if let Some((stored_hash, _, stored_etag)) =
                        self.fetch_stored_calendar_item(&href).await
                    {
                        if stored_hash != hash {
                            self.storage
                                .update_item(&href, &stored_etag, &calendar_data.into())
                                .await?;
                        }
                    } else {
                        self.storage
                            .create_item(
                                &self.collection.to_string(),
                                &calendar_data.into(),
                                CreateItemOptions {
                                    resource_name: Some(id.clone()),
                                },
                            )
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }
}
