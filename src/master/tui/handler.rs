use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::{App, AppResult};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }

        // Navigation
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
            app.select();
        }
        KeyCode::Left | KeyCode::Char('h') => {
            app.back();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
