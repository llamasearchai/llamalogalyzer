//! TUI for LlamaLogAnalyzer MLX Edition
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Terminal,
};
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use crate::parsers::LogEntry;
use crate::analyzer::statistics::{LogStatistics, compute_statistics};
use crate::ml::anomaly::{AnomalyDetector, AnomalyResult};

enum DisplayMode {
    Dashboard,
    LogViewer,
    Analysis,
    Help,
}

pub struct TUIApp {
    pub log_file: Option<PathBuf>,
    pub running: bool,
    pub mode: DisplayMode,
    pub input: String,
    pub log_entries: Vec<LogEntry>,
    pub statistics: Option<LogStatistics>,
    pub anomalies: Vec<AnomalyResult>,
    pub selected_log_index: usize,
    pub scroll_offset: usize,
}

impl TUIApp {
    pub fn new(log_file: Option<PathBuf>) -> Self {
        Self {
            log_file,
            running: true,
            mode: DisplayMode::Dashboard,
            input: String::new(),
            log_entries: Vec::new(),
            statistics: None,
            anomalies: Vec::new(),
            selected_log_index: 0,
            scroll_offset: 0,
        }
    }
    
    pub fn run(&mut self) -> Result<()> {
        // Load log entries if file specified
        if let Some(file) = &self.log_file {
            if let Ok(entries) = futures::executor::block_on(crate::parsers::parse_log_file(file)) {
                self.log_entries = entries;
                self.statistics = Some(compute_statistics(&self.log_entries));
                
                // Detect anomalies
                let detector = AnomalyDetector::new(0.7);
                self.anomalies = detector.detect_anomalies(&self.log_entries);
            }
        }
        
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        
        // Main loop
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(100);
        
        while self.running {
            terminal.draw(|f| self.ui(f))?;
            
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
                
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                }
            }
            
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
        
        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        
        Ok(())
    }
    
    fn ui<B: ratatui::backend::Backend>(&self, f: &mut ratatui::Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)].as_ref())
            .split(f.size());
            
        self.render_tabs(f, chunks[0]);
        
        match self.mode {
            DisplayMode::Dashboard => self.render_dashboard(f, chunks[1]),
            DisplayMode::LogViewer => self.render_log_viewer(f, chunks[1]),
            DisplayMode::Analysis => self.render_analysis(f, chunks[1]),
            DisplayMode::Help => self.render_help(f, chunks[1]),
        }
        
        self.render_status_bar(f, chunks[2]);
    }
    
    fn render_tabs<B: ratatui::backend::Backend>(&self, f: &mut ratatui::Frame<B>, area: Rect) {
        let titles = vec!["Dashboard", "Logs", "Analysis", "Help"];
        let mode_index = match self.mode {
            DisplayMode::Dashboard => 0,
            DisplayMode::LogViewer => 1,
            DisplayMode::Analysis => 2,
            DisplayMode::Help => 3,
        };
        
        let tab_titles: Vec<Spans> = titles
            .iter()
            .map(|t| {
                Spans::from(Span::styled(
                    *t,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
            })
            .collect();
            
        let tabs = Tabs::new(tab_titles)
            .select(mode_index)
            .block(Block::default().borders(Borders::ALL).title("LlamaLogAnalyzer MLX"))
            .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
            
        f.render_widget(tabs, area);
    }
    
    fn render_dashboard<B: ratatui::backend::Backend>(&self, f: &mut ratatui::Frame<B>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Dashboard")
            .border_style(Style::default().fg(Color::Cyan));
            
        f.render_widget(block, area);
        
        let inner_area = area.inner(&Constraint::Horizontal(2));
        
        let dashboard_text = if let Some(stats) = &self.statistics {
            let mut content = Vec::new();
            content.push(Spans::from(vec![
                Span::styled("Log Summary", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]));
            content.push(Spans::from(""));
            
            content.push(Spans::from(vec![
                Span::raw("Total Entries: "),
                Span::styled(format!("{}", stats.total_entries), Style::default().fg(Color::Yellow)),
            ]));
            
            content.push(Spans::from(""));
            content.push(Spans::from(vec![
                Span::styled("Log Levels:", Style::default().fg(Color::Green)),
            ]));
            
            for (level, count) in &stats.level_counts {
                let color = match level.as_str() {
                    "ERROR" => Color::Red,
                    "WARNING" => Color::Yellow,
                    "INFO" => Color::Blue,
                    "DEBUG" => Color::Gray,
                    _ => Color::White,
                };
                
                content.push(Spans::from(vec![
                    Span::raw("  - "),
                    Span::styled(level.clone(), Style::default().fg(color)),
                    Span::raw(": "),
                    Span::styled(format!("{}", count), Style::default().fg(Color::Yellow)),
                ]));
            }
            
            if let Some(time_span) = &stats.time_span {
                content.push(Spans::from(""));
                content.push(Spans::from(vec![
                    Span::styled("Time Span:", Style::default().fg(Color::Green)),
                ]));
                content.push(Spans::from(vec![
                    Span::raw("  First Entry: "),
                    Span::styled(
                        time_span.first_entry.format("%Y-%m-%d %H:%M:%S").to_string(),
                        Style::default().fg(Color::Yellow),
                    ),
                ]));
                content.push(Spans::from(vec![
                    Span::raw("  Last Entry: "),
                    Span::styled(
                        time_span.last_entry.format("%Y-%m-%d %H:%M:%S").to_string(),
                        Style::default().fg(Color::Yellow),
                    ),
                ]));
                content.push(Spans::from(vec![
                    Span::raw("  Duration: "),
                    Span::styled(
                        format!("{} seconds", time_span.duration_seconds),
                        Style::default().fg(Color::Yellow),
                    ),
                ]));
            }
            
            if !self.anomalies.is_empty() {
                content.push(Spans::from(""));
                content.push(Spans::from(vec![
                    Span::styled("Detected Anomalies:", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                ]));
                
                for (i, anomaly) in self.anomalies.iter().enumerate() {
                    content.push(Spans::from(vec![
                        Span::raw(format!("  {}. ", i+1)),
                        Span::styled(
                            &anomaly.description,
                            Style::default().fg(Color::Red),
                        ),
                    ]));
                    content.push(Spans::from(vec![
                        Span::raw("     Confidence: "),
                        Span::styled(
                            format!("{:.1}%", anomaly.confidence * 100.0),
                            Style::default().fg(Color::Yellow),
                        ),
                        Span::raw(" | Severity: "),
                        Span::styled(
                            format!("{}/5", anomaly.severity),
                            Style::default().fg(Color::Red),
                        ),
                    ]));
                }
            }
            
            content
        } else {
            vec![
                Spans::from(vec![
                    Span::styled("No log file loaded", Style::default().fg(Color::Yellow)),
                ]),
                Spans::from(""),
                Spans::from("Press 'o' to open a log file or use the 'file' parameter when starting the application."),
            ]
        };
        
        let paragraph = Paragraph::new(dashboard_text)
            .block(Block::default())
            .wrap(Wrap { trim: true });
            
        f.render_widget(paragraph, inner_area);
    }
    
    fn render_log_viewer<B: ratatui::backend::Backend>(&self, f: &mut ratatui::Frame<B>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Log Viewer")
            .border_style(Style::default().fg(Color::Blue));
            
        f.render_widget(block, area);
        
        let inner_area = area.inner(&Constraint::Horizontal(2));
        
        if self.log_entries.is_empty() {
            let text = vec![
                Spans::from(vec![
                    Span::styled("No log entries to display", Style::default().fg(Color::Yellow)),
                ]),
            ];
            
            let paragraph = Paragraph::new(text)
                .block(Block::default());
                
            f.render_widget(paragraph, inner_area);
            return;
        }
        
        // Create list items for logs with appropriate styling
        let items: Vec<ListItem> = self.log_entries
            .iter()
            .skip(self.scroll_offset)
            .take(inner_area.height as usize)
            .enumerate()
            .map(|(i, entry)| {
                let content = if let Some(timestamp) = entry.timestamp {
                    format!("{} [{}] {}", 
                        timestamp.format("%Y-%m-%d %H:%M:%S"),
                        entry.level,
                        entry.message
                    )
                } else {
                    format!("[{}] {}", entry.level, entry.message)
                };
                
                let color = match entry.level.as_str() {
                    "ERROR" => Color::Red,
                    "WARNING" => Color::Yellow,
                    "INFO" => Color::Blue,
                    "DEBUG" => Color::Gray,
                    _ => Color::White,
                };
                
                let style = if i + self.scroll_offset == self.selected_log_index {
                    Style::default().fg(color).bg(Color::DarkGray).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(color)
                };
                
                ListItem::new(Spans::from(Span::styled(content, style)))
            })
            .collect();
            
        let list = List::new(items)
            .block(Block::default());
            
        f.render_widget(list, inner_area);
    }
    
    fn render_analysis<B: ratatui::backend::Backend>(&self, f: &mut ratatui::Frame<B>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Analysis")
            .border_style(Style::default().fg(Color::Magenta));
            
        f.render_widget(block, area);
        
        // Simplified placeholder for analysis view
        let text = vec![
            Spans::from(vec![
                Span::styled("Log Analysis Results", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]),
            Spans::from(""),
            Spans::from("This is a placeholder for more detailed analysis."),
            Spans::from("In the full implementation, this would include:"),
            Spans::from(""),
            Spans::from("- Time series visualizations"),
            Spans::from("- Pattern detection results"),
            Spans::from("- Correlation between different log types"),
            Spans::from("- ML-powered anomaly insights"),
        ];
        
        let paragraph = Paragraph::new(text)
            .block(Block::default())
            .wrap(Wrap { trim: true });
            
        f.render_widget(paragraph, area.inner(&Constraint::Horizontal(2)));
    }
    
    fn render_help<B: ratatui::backend::Backend>(&self, f: &mut ratatui::Frame<B>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Help")
            .border_style(Style::default().fg(Color::Gray));
            
        f.render_widget(block, area);
        
        let text = vec![
            Spans::from(vec![
                Span::styled("LlamaLogAnalyzer MLX Edition Help", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("Navigation:", Style::default().fg(Color::Green)),
            ]),
            Spans::from("  Tab          - Switch between views"),
            Spans::from("  q            - Quit application"),
            Spans::from("  h            - Show this help"),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("Log Viewer:", Style::default().fg(Color::Green)),
            ]),
            Spans::from("  ↑/↓          - Navigate log entries"),
            Spans::from("  PgUp/PgDown  - Scroll multiple entries"),
            Spans::from("  Home/End     - Jump to start/end"),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("File Operations:", Style::default().fg(Color::Green)),
            ]),
            Spans::from("  o            - Open log file"),
            Spans::from("  r            - Reload current file"),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("Analysis:", Style::default().fg(Color::Green)),
            ]),
            Spans::from("  a            - Run analysis on current logs"),
            Spans::from("  f            - Filter logs (not implemented)"),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("About:", Style::default().fg(Color::Blue)),
            ]),
            Spans::from("  LlamaLogAnalyzer MLX Edition v2.0.0"),
            Spans::from("  Optimized for Apple Silicon with MLX acceleration"),
        ];
        
        let paragraph = Paragraph::new(text)
            .block(Block::default())
            .wrap(Wrap { trim: true });
            
        f.render_widget(paragraph, area.inner(&Constraint::Horizontal(2)));
    }
    
    fn render_status_bar<B: ratatui::backend::Backend>(&self, f: &mut ratatui::Frame<B>, area: Rect) {
        let file_info = if let Some(path) = &self.log_file {
            format!("File: {}", path.display())
        } else {
            "No file loaded".to_string()
        };
        
        let entries_info = format!("Entries: {}", self.log_entries.len());
        
        let help_hint = "Press 'h' for help | 'q' to quit";
        
        let status_text = Spans::from(vec![
            Span::styled(file_info, Style::default().fg(Color::Blue)),
            Span::raw(" | "),
            Span::styled(entries_info, Style::default().fg(Color::Green)),
            Span::raw(" | "),
            Span::styled(help_hint, Style::default().fg(Color::Gray)),
        ]);
        
        let paragraph = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default());
            
        f.render_widget(paragraph, area);
    }
    
    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.running = false,
            KeyCode::Char('h') => self.mode = DisplayMode::Help,
            KeyCode::Tab => {
                self.mode = match self.mode {
                    DisplayMode::Dashboard => DisplayMode::LogViewer,
                    DisplayMode::LogViewer => DisplayMode::Analysis,
                    DisplayMode::Analysis => DisplayMode::Help,
                    DisplayMode::Help => DisplayMode::Dashboard,
                }
            },
            KeyCode::Char('1') => self.mode = DisplayMode::Dashboard,
            KeyCode::Char('2') => self.mode = DisplayMode::LogViewer,
            KeyCode::Char('3') => self.mode = DisplayMode::Analysis,
            KeyCode::Char('4') => self.mode = DisplayMode::Help,
            
            // Log navigation
            KeyCode::Up if self.mode == DisplayMode::LogViewer => {
                if self.selected_log_index > 0 {
                    self.selected_log_index -= 1;
                    // Adjust scroll if needed
                    if self.selected_log_index < self.scroll_offset {
                        self.scroll_offset = self.selected_log_index;
                    }
                }
            },
            KeyCode::Down if self.mode == DisplayMode::LogViewer => {
                if self.selected_log_index < self.log_entries.len() - 1 {
                    self.selected_log_index += 1;
                    // Adjust scroll if needed (assuming visible area is 10 lines)
                    if self.selected_log_index >= self.scroll_offset + 10 {
                        self.scroll_offset = self.selected_log_index - 9;
                    }
                }
            },
            // Other key handlers...
            _ => {}
        }
    }
} 