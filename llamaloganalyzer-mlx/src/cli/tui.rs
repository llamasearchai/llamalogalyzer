//! TUI for LlamaLogAnalyzer MLX Edition
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use std::io;
use std::time::Duration;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph, Tabs},
    Terminal,
};

pub fn run_tui() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App state
    let mut running = true;
    let mut tab_index = 0;

    // Main loop
    while running {
        terminal.draw(|f| {
            let size = f.size();
            
            // Create the layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),  // Tabs
                    Constraint::Min(5),     // Content
                    Constraint::Length(3),  // Status
                ].as_ref())
                .split(size);

            // Create tabs
            let tab_titles = vec!["Dashboard", "Logs", "Analysis"];
            let tabs = Tabs::new(
                tab_titles.iter().map(|t| {
                    Line::from(Span::styled(*t, Style::default().fg(Color::White)))
                }).collect())
                .select(tab_index)
                .block(Block::default().borders(Borders::ALL).title("LlamaLogAnalyzer MLX"));
            
            f.render_widget(tabs, chunks[0]);

            // Content based on selected tab
            let content = match tab_index {
                0 => "Dashboard View - Welcome to LlamaLogAnalyzer MLX Edition",
                1 => "Log Analysis View - Select a log file to analyze",
                2 => "Advanced Analysis - ML-powered anomaly detection",
                _ => "Unknown tab",
            };

            let paragraph = Paragraph::new(content)
                .block(Block::default().borders(Borders::ALL).title("Content"));
            
            f.render_widget(paragraph, chunks[1]);

            // Status bar
            let status = Paragraph::new("Press 'q' to quit, Tab to switch views")
                .block(Block::default().borders(Borders::ALL).title("Status"));
            
            f.render_widget(status, chunks[2]);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => running = false,
                    KeyCode::Tab => tab_index = (tab_index + 1) % 3,
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    
    Ok(())
}
