use crossterm::event::{self, Event};
use ratatui::widgets::ListState;
use ratatui::{backend::Backend, Terminal};

#[derive(Debug, Default)]
pub struct App {
    pub running: bool,
    pub spacex_data: Vec<crate::data::Launch>,
    pub filtered_data: Vec<crate::data::Launch>,
    pub selected_index: usize,
    pub list_state: ListState,
    pub search_mode: bool,
    pub search_query: String,
    pub filtered: bool, // whenter the data is filtered
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            running: true,
            spacex_data: Vec::new(),
            filtered_data: Vec::new(),
            selected_index: 0,
            list_state,
            search_mode: false,
            search_query: String::new(),
            filtered: false,
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

    pub fn initialize_data(&mut self) {
        self.filtered_data = self.spacex_data.clone();
        self.selected_index = 0;
        self.list_state.select(Some(0));
        self.filtered = false;
    }

    pub fn filter_data(&mut self) {
        self.filtered_data = self
            .spacex_data
            .iter()
            .filter(|launch| {
                launch
                    .name
                    .to_lowercase()
                    .contains(&self.search_query.to_lowercase())
            })
            .cloned()
            .collect();
        self.selected_index = 0;
        self.list_state.select(Some(0));
        self.filtered = true;
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
