use crate::app::App;
use crossterm::event::KeyEvent;

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        crossterm::event::KeyCode::Up => {
            if let Some(selected) = app.list_state.selected() {
                if selected > 0 {
                    app.list_state.select(Some(selected - 1));
                    app.selected_index -= 1;
                }
            }
        }
        crossterm::event::KeyCode::Down => {
            if let Some(selected) = app.list_state.selected() {
                if selected + 1 < app.spacex_data.len() {
                    app.list_state.select(Some(selected + 1));
                    app.selected_index += 1;
                }
            }
        }
        crossterm::event::KeyCode::Char('q') => app.quit(),
        _ => {}
    }
}
