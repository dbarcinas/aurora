use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        // enter search mode
        KeyCode::Char('/') => {
            app.search_mode = true;
            app.search_query.clear(); // clear the query
        }
        // handle search input
        KeyCode::Char(c) if app.search_mode => {
            app.search_query.push(c);
        }
        // remove last character from the search query
        KeyCode::Backspace if app.search_mode => {
            app.search_query.pop();
        }
        // confirm search
        KeyCode::Enter if app.search_mode => {
            app.search_mode = false;
            app.filter_data();
        }
        // cancel search/reset filtered data
        KeyCode::Esc => {
            if app.filtered {
                // reset to original data
                app.initialize_data();
            } else if app.search_mode {
                app.search_mode = false;
            }
        }
        // navigate up
        KeyCode::Up => {
            if let Some(selected) = app.list_state.selected() {
                if selected > 0 {
                    app.list_state.select(Some(selected - 1));
                    app.selected_index = selected - 1;
                }
            }
        }
        // navigate down
        KeyCode::Down => {
            if let Some(selected) = app.list_state.selected() {
                if selected + 1 < app.spacex_data.len() {
                    app.list_state.select(Some(selected + 1));
                    app.selected_index = selected + 1;
                }
            }
        }
        // quit the app
        KeyCode::Char('q') => app.quit(),
        _ => {}
    }
}
