use engine::simulator::Simulator;
use ratatui::crossterm::event::{self, Event, DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::{Frame, Terminal};
use std::io;
use std::error::Error;

enum UIState {
    StartGameQuery,
    ShowDealer,
    Done,
}

struct App {
    simulator: Simulator,
    state: UIState,
    buffer: String,
}

impl App {
    fn new() -> Self {
        Self {
            simulator: Simulator::new(),
            state: UIState::StartGameQuery,
            buffer: String::new(),
        }
    }

    fn handle_input(&mut self) {
        use UIState::*;
        match self.state {
            StartGameQuery => {
                match &self.buffer[0..1] {
                    "y" | "Y" => {
                        // self.simulator = self.simulator.choose_dealer();
                        self.state = ShowDealer;
                    },
                    "n" | "N" => {
                        self.state = Done;
                    },
                    _ => {
                        self.buffer.clear();
                    }
                }
            },
            ShowDealer => {
                self.state = Done;
            },
            _ => {},
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    enable_raw_mode()?;
    let stdout = io::stdout();
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(res?)
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        if matches!(app.state, UIState::Done) {
            break;
        }
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match key.code {
                event::KeyCode::Esc => {
                    break;
                },
                event::KeyCode::Enter => {
                    app.handle_input();
                },
                event::KeyCode::Backspace => {
                    if app.buffer.len() > 0 {
                        app.buffer.pop();
                    }
                },
                event::KeyCode::Char(value) => {
                    app.buffer.push(value);
                },
                _ => {},
            }
        }
    }
    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    use UIState::*;
    match app.state {
        StartGameQuery => {
            frame.render_widget(format!("Would you like to start a new game? [Y/n] {}", app.buffer), frame.area());
        },
        ShowDealer => {
            frame.render_widget("Dealer chosen. Press Enter", frame.area());
        },
        _ => {}
    }
}
