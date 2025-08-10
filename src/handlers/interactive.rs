use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

pub fn select_host_interactively(hosts: Vec<&crate::config::HostConfig>) -> Option<String> {
    if hosts.is_empty() {
        println!("No hosts configured");
        return None;
    }

    if hosts.len() == 1 {
        return Some(hosts[0].name.clone());
    }

    // Initialize terminal
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    
    let mut stdout = io::stdout();
    let mut selected_index = 0;
    
    // Print instructions
    execute!(
        stdout,
        Clear(ClearType::All),
        crossterm::cursor::MoveTo(0, 0),
        SetForegroundColor(Color::Cyan),
        Print("Select a host to connect to:\n"),
        Print("(Use ↑/↓ arrows to navigate, Enter to select, Esc/Ctrl+C to cancel)\n"),
        Print("\n"), // Extra newline for spacing
        ResetColor
    ).expect("Failed to print instructions");
    
    loop {
        // Move cursor to the start of the host list (after the instructions)
        execute!(stdout, crossterm::cursor::MoveTo(0, 3)).expect("Failed to move cursor");
        
        // Print hosts with selection indicator
        for (i, host) in hosts.iter().enumerate() {
            // Move to the correct line for this host
            execute!(stdout, crossterm::cursor::MoveTo(0, (3 + i) as u16)).expect("Failed to move cursor");
            
            // Clear the line first to ensure proper redraw
            execute!(stdout, Clear(ClearType::CurrentLine)).expect("Failed to clear line");
            
            if i == selected_index {
                execute!(
                    stdout,
                    SetBackgroundColor(Color::Blue),
                    SetForegroundColor(Color::White),
                    Print(format!("▶ {:<20} {}@{}:{}", host.name, host.username, host.hostname, host.port)),
                    ResetColor
                ).expect("Failed to print");
            } else {
                execute!(
                    stdout,
                    ResetColor,
                    Print(format!("  {:<20} {}@{}:{}", host.name, host.username, host.hostname, host.port)),
                    ResetColor
                ).expect("Failed to print");
            }
        }
        
        // Clear any remaining lines from previous renders
        for i in hosts.len()..(hosts.len() + 5) {
            execute!(stdout, crossterm::cursor::MoveTo(0, (3 + i) as u16)).expect("Failed to move cursor");
            execute!(stdout, Clear(ClearType::CurrentLine)).expect("Failed to clear line");
        }
        
        // Move cursor back to the start of the list for the next iteration
        execute!(stdout, crossterm::cursor::MoveTo(0, 3)).expect("Failed to move cursor");
        stdout.flush().expect("Failed to flush stdout");
        
        // Read user input
        match event::read() {
            Ok(Event::Key(KeyEvent { code, modifiers, .. })) => {
                match code {
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected_index < hosts.len() - 1 {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        // Restore terminal
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        return Some(hosts[selected_index].name.clone());
                    }
                    KeyCode::Esc => {
                        // Restore terminal
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        return None;
                    }
                    KeyCode::Char('c') if modifiers == KeyModifiers::CONTROL => {
                        // Handle Ctrl+C
                        execute!(stdout, crossterm::cursor::MoveTo(0, (3 + hosts.len()) as u16)).expect("Failed to move cursor");
                        execute!(stdout, Clear(ClearType::CurrentLine)).expect("Failed to clear line");
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        println!("\nCancelled by user (Ctrl+C)");
                        return None;
                    }
                    _ => {}
                }
            }
            Err(_) => {
                // Restore terminal on error
                terminal::disable_raw_mode().expect("Failed to disable raw mode");
                return None;
            }
            _ => {} // Handle other events (focus, mouse, etc.) by ignoring them
        }
    }
}