use std::error::Error;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::{fs, thread};
use std::time::Duration;
use serde::Deserialize;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

#[derive(Deserialize)]
struct Config {
    host: String,
    port: u16,
}

fn load_config() -> Config {
    let config_str = fs::read_to_string("config.toml").expect("config.toml not found");
    toml::from_str(&config_str)
        .expect("Failed to parse config.toml")
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
           stream: &mut TcpStream,
           rx: mpsc::Receiver<String>,
) -> Result<(), Box<dyn Error>> {
    let mut messages: Vec<String> = vec!["[system] Welcome to Ratroom".to_string()];
    let mut input = String::new();

    loop {
        while let Ok(message) = rx.try_recv() {
            messages.push(message);
        }
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(frame.area());

            let header = Paragraph::new(
                "Ratroom — Enter=send | /who=list users | /quit or Esc=exit"
            )
                .style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                );
            let message_lines: Vec<Line> = messages
                .iter()
                .map(|msg| {
                    if msg.starts_with("[system]") {
                        Line::from(Span::styled(
                            msg.clone(),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::ITALIC),
                        ))
                    } else {
                        Line::from(Span::raw(msg.clone()))
                    }
                })
                .collect();



            let messages_widget = Paragraph::new(message_lines)
                .block(Block::default().title("Messages").borders(Borders::ALL))
                .wrap(Wrap { trim: false });

            let input_widget = Paragraph::new(input.as_str())
                .block(Block::default().title("Input").borders(Borders::ALL));

            frame.render_widget(header, chunks[0]);
            frame.render_widget(messages_widget, chunks[1]);
            frame.render_widget(input_widget, chunks[2]);

            frame.set_cursor_position(Position {
                x: chunks[1].x + input.len() as u16 + 1,
                y: chunks[1].y + 1,
            });
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => {
                            input.push(c);
                        }
                        KeyCode::Backspace => {
                            input.pop();
                        }
                        KeyCode::Enter => {
                            let trimmed = input.trim().to_string();
                            if trimmed == "/quit" {
                                break;
                            }
                            if !trimmed.is_empty() {
                                stream.write_all(trimmed.as_bytes())?;
                            }
                            input.clear();
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config();
    let addr = format!("{}:{}", config.host, config.port);

    let mut stream = TcpStream::connect(&addr)?;
    println!("Connected to server");

    let username = loop {
        let mut input = String::new();
        println!("Enter your user-name: ");
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed.is_empty() {
            println!("Username cannot be empty");
            continue;
        }
        break trimmed.to_string()
    };

    stream.write_all(username.as_bytes()).unwrap();

    println!("Welcome {username}!");

    println!("Type messages and press Enter to send.");
    println!("Type /quit to exit.");

    let mut read_stream = stream.try_clone()?;

    let(tx, rx) = mpsc::channel::<String>();

    thread::spawn(move ||{
       loop {
           let mut buffer = [0; 1024];

           let bytes_read = match read_stream.read(&mut buffer) {
               Ok(0) => {
                   let _ = tx.send("[system] Server closed the connection".to_string());
                   return;
               }
               Ok(n) => n,
               Err(e) => {
                   let _ = tx.send(format!("[system] Failed to read from server: {e}"));
                   return;
               }
           };
           let message =  String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
           let _ = tx.send(message);
       }
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut stream, rx);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}