use crossterm::event::{self, Event};
use ratatui::widgets::ListState;
use ratatui::{backend::Backend, Terminal};

#[derive(Debug, Default)]
pub struct App {
    pub running: bool,
    pub spacex_data: Vec<crate::data::Launch>,
    pub selected_index: usize,
    pub list_state: ListState,
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            running: true,
            spacex_data: Vec::new(),
            selected_index: 0,
            list_state,
        }
    }

    pub fn run<B: Backend>(&mut self, mut terminal: Terminal<B>) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| crate::ui::draw(frame, self))?;
            if let Event::Key(key) = event::read()? {
                crate::events::handle_key_event(self, key);
            }
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
