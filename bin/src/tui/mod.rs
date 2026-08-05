mod cli;
mod widget;

pub use cli::*;
pub use widget::*;

use alleged_lib::graph::Graph;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
};
use std::{io, sync::Arc};

pub struct Tui {
    graph: Arc<Graph>,
    exit: bool,
}

impl Tui {
    #[must_use]
    pub fn new(graph: &Arc<Graph>) -> Self {
        Self {
            graph: Arc::clone(graph),
            exit: false,
        }
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    const fn exit(&mut self) {
        self.exit = true;
    }
    const fn handle_key_event(&mut self, key_event: KeyEvent) {
        if matches!(key_event.code, KeyCode::Char('q')) {
            self.exit();
        }
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }
}
