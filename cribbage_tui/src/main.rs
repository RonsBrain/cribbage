use cribbage_engine::card::{Card, Rank, Suit};
use cribbage_engine::engine::{EngineState, Player, new_engine};
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::{Backend, CrosstermBackend, Terminal};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    run(&mut terminal)?;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<bool> {
    let mut engine = new_engine();
    loop {
        terminal.draw(|f| ui(f, &engine))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match engine {
                EngineState::NewGame(ref start) => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                        engine = start.choose_dealer();
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        break;
                    }
                    _ => {}
                },
                _ => {
                    break;
                }
            }
        }
    }
    Ok(true)
}

fn format_card(card: &Card) -> String {
    use Rank::*;
    use Suit::*;
    let mut result = String::new();
    result.push_str(match card.rank {
        Ace => "A",
        Two => "2",
        Three => "3",
        Four => "4",
        Five => "5",
        Six => "6",
        Seven => "7",
        Eight => "8",
        Nine => "9",
        Ten => "10",
        Jack => "J",
        Queen => "Q",
        King => "K",
    });
    result.push_str(match card.suit {
        Spades => "♠",
        Hearts => "♡",
        Clubs => "♣",
        Diamonds => "♢",
    });
    result
}

fn ui(frame: &mut Frame, engine: &EngineState) {
    match engine {
        EngineState::NewGame(_) => {
            frame.render_widget("Start a new game? [Y/n]", frame.area());
        }
        EngineState::DealerChosen(result) => {
            let [player_ui, cpu_ui, dealer_ui] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ])
                .areas(frame.area());
            frame.render_widget(
                format!("You cut {}", format_card(&result.cut_cards[&Player::First])),
                player_ui,
            );
            frame.render_widget(
                format!("I cut {}", format_card(&result.cut_cards[&Player::Second])),
                cpu_ui,
            );
            let dealer = match result.dealer {
                Player::First => "You are",
                Player::Second => "I am",
            };
            frame.render_widget(format!("{} the dealer", dealer), dealer_ui);
        }
    }
}
