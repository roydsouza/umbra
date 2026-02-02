use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Clear},
    text::{Line, Span},
    Frame,
};

use crate::app::App;
use crate::theme::*;
use crate::data::logs::LogLevel;

fn colorize_log<'a>(content: &'a str, base_style: Style) -> Line<'a> {
    let parts: Vec<&str> = content.split("[scrubbed]").collect();
    let mut spans = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        if !part.is_empty() {
             spans.push(Span::styled(*part, base_style));
        }
        if i < parts.len() - 1 {
            spans.push(Span::styled("[scrubbed]", Style::default().fg(TOKYO_PURPLE)));
        }
    }
    Line::from(spans)
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(8), // Top Row (Arti Status | Onion Services)
            Constraint::Length(8), // Middle Row (Dependent Apps | Privacy Monitor)
            Constraint::Min(10),   // Logs
            Constraint::Length(1), // Footer
        ])
        .split(f.size());

    // Header
    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TOKYO_BORDER))
        .title(" A-WATCHER v0.1.0  [Privacy: 🟢] ")
        .title_style(Style::default().fg(TOKYO_BLUE).bold());
    
    f.render_widget(header_block, chunks[0]);

    // Top Row
    let top_row_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

    let arti_status_text = if app.arti_status.running {
        vec![
            Line::from(vec![Span::raw("Status: "), Span::styled("🟢 ONLINE", Style::default().fg(TOKYO_GREEN).bold())]),
            Line::from(format!("PID:    {}", app.arti_status.pid.unwrap_or(0))),
            Line::from(format!("Mem:    {:.1} MB", app.arti_status.memory_mb)),
            Line::from(format!("CPU:    {:.1}%", app.arti_status.cpu_percent)),
            Line::from(vec![
                Span::raw("SOCKS:  "), 
                if app.socks_active { 
                    Span::styled("🟢 :9050 OK", Style::default().fg(TOKYO_GREEN)) 
                } else { 
                    Span::styled("🔴 :9050 FAIL", Style::default().fg(TOKYO_RED)) 
                }
            ]),
        ]
    } else {
        vec![
             Line::from(vec![Span::raw("Status: "), Span::styled("🔴 OFFLINE", Style::default().fg(TOKYO_RED).bold())]),
             Line::from("Process 'arti' not found"),
        ]
    };

    let arti_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TOKYO_BORDER))
        .title(" Arti Status ");
    f.render_widget(Paragraph::new(arti_status_text).block(arti_block).style(Style::default().fg(TOKYO_FG)), top_row_chunks[0]);

    let onion_text: Vec<Line> = if app.onion_services.is_empty() {
        vec![Line::from(Span::styled("No services found", Style::default().fg(TOKYO_FG)))]
    } else {
        app.onion_services.iter().map(|s| {
            let status = match s.status {
                crate::data::onion::OnionStatus::Active => Span::styled("🟢", Style::default().fg(TOKYO_GREEN)),
                crate::data::onion::OnionStatus::Pending => Span::styled("🟡", Style::default().fg(TOKYO_ORANGE)),
                crate::data::onion::OnionStatus::Error => Span::styled("🔴", Style::default().fg(TOKYO_RED)),
            };
            // Truncate hostname if needed or just show
            let host = s.hostname.as_deref().unwrap_or("pending...");
            Line::from(vec![
                status, 
                Span::raw(" "), 
                Span::styled(&s.name, Style::default().fg(TOKYO_BLUE).bold()),
                Span::raw(": "),
                Span::styled(host, Style::default().fg(TOKYO_FG))
            ])
        }).collect()
    };

    let onion_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TOKYO_BORDER))
        .title(" Onion Services ");
    f.render_widget(Paragraph::new(onion_text).block(onion_block).style(Style::default().fg(TOKYO_FG)), top_row_chunks[1]);

    // Middle Row
    let mid_row_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[2]);
        
    let deps_text: Vec<Line> = if app.dependent_apps.is_empty() {
        vec![Line::from("No active connections")]
    } else {
        app.dependent_apps.iter().map(|app| {
             Line::from(vec![
                 Span::styled("🟢 ", Style::default().fg(TOKYO_GREEN)),
                 Span::styled(&app.name, Style::default().fg(TOKYO_FG).bold()),
                 Span::raw(format!(" ({})", app.pid))
             ])
        }).collect()
    };
        
    let deps_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TOKYO_BORDER))
        .title(" Dependent Apps ");
    f.render_widget(Paragraph::new(deps_text).block(deps_block).style(Style::default().fg(TOKYO_FG)), mid_row_chunks[0]);
    
    let privacy_text = vec![
        Line::from(vec![Span::styled("🛡️  Privacy Shield", Style::default().fg(TOKYO_PURPLE).bold())]),
        Line::from(""),
        Line::from(format!("Errors:   {}", app.log_stats.error_count)),
        Line::from(format!("Warnings: {}", app.log_stats.warn_count)),
        Line::from(format!("Scrubbed: {}", app.log_stats.scrub_count)),
    ];

    let privacy_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TOKYO_BORDER))
        .title(" Privacy Monitor ");
    f.render_widget(Paragraph::new(privacy_text).block(privacy_block).style(Style::default().fg(TOKYO_FG)), mid_row_chunks[1]);

    // Log Panel
    let logs_to_show: Vec<Line> = if app.logs.is_empty() {
        vec![Line::from("Waiting for logs...")]
    } else {
        app.logs.iter()
            .skip(app.log_scroll)
            .take(chunks[3].height as usize)
            .map(|log| {
                let style = match log.level {
                    LogLevel::Info => Style::default().fg(TOKYO_FG),
                    LogLevel::Warn => Style::default().fg(TOKYO_ORANGE),
                    LogLevel::Error => Style::default().fg(TOKYO_RED),
                    LogLevel::Debug => Style::default().fg(TOKYO_BORDER),
                    _ => Style::default().fg(TOKYO_FG),
                };
                colorize_log(&log.content, style)
            })
            .collect()
    };
    
    let log_title = if app.log_auto_scroll { 
        " Logs (arti.log) [FOLLOW] " 
    } else { 
        " Logs (arti.log) " 
    };

    let log_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TOKYO_BORDER))
        .title(log_title);
    f.render_widget(Paragraph::new(logs_to_show).block(log_block), chunks[3]);

    // Footer
    let footer_text = " [q] Quit  [s] Start  [x] Stop  [r] Restart  [?] Help";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(TOKYO_FG));
    f.render_widget(footer, chunks[4]);

    // Confirmation Modal
    if let Some(action) = app.confirm_action {
        let area = centered_rect(60, 20, f.size());
        f.render_widget(Clear, area);
        
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(TOKYO_RED))
            .title(" Confirm Action ");
            
        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("Are you sure you want to "),
                Span::styled(format!("{:?}", action).to_uppercase(), Style::default().fg(TOKYO_RED).bold()),
                Span::raw(" Arti?"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" [y] Yes ", Style::default().bg(TOKYO_RED).fg(Color::Black)),
                Span::raw("   "),
                Span::styled(" [n] No ", Style::default().fg(TOKYO_FG)),
            ]),
        ];
        
        f.render_widget(Paragraph::new(text).block(block).alignment(Alignment::Center), area);
    }
    
    // Notification Toast
    if let Some((msg, _)) = &app.notification {
        let area = centered_rect(50, 15, f.size());
        f.render_widget(Clear, area);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(TOKYO_ORANGE))
            .title(" ⚠ Notice ");
            
        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(msg.as_str(), Style::default().fg(TOKYO_ORANGE).bold())
            ])
        ];
        
        f.render_widget(Paragraph::new(text).block(block).alignment(Alignment::Center), area);
    }
}
