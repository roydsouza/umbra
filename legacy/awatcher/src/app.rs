use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{backend::Backend, Terminal};
use std::time::Duration;
use tokio::sync::mpsc;

use crate::ui::ui;
use crate::data::process::{ArtiMonitor, ArtiStatus};
use crate::data::socks::test_socks_proxy;
use crate::data::logs::{tail_file, LogLine, LogLevel};
use crate::data::config::read_onion_services;
use crate::data::onion::{OnionScanner, OnionService};
use crate::data::deps::{scan_dependent_apps, DependentApp};
use crate::actions::Action;

pub enum AppEvent {
    ArtiUpdate(ArtiStatus),
    SocksUpdate(bool),
    LogUpdate(Vec<LogLine>),
    OnionUpdate(Vec<OnionService>),
    DepsUpdate(Vec<DependentApp>),
}

#[derive(PartialEq)]
pub enum KeyState {
    Normal,
    WaitingForG(std::time::Instant),
}

pub struct LogStats {
    pub error_count: usize,
    pub warn_count: usize,
    pub scrub_count: usize,
}

impl Default for LogStats {
    fn default() -> Self {
        Self { error_count: 0, warn_count: 0, scrub_count: 0 }
    }
}

pub struct App {
    pub should_quit: bool,
    pub arti_status: ArtiStatus,
    pub socks_active: bool,
    pub logs: Vec<LogLine>,
    pub log_scroll: usize,
    pub log_auto_scroll: bool,
    pub onion_services: Vec<OnionService>,
    pub dependent_apps: Vec<DependentApp>,
    pub confirm_action: Option<Action>,
    pub notification: Option<(String, std::time::Instant)>,
    pub log_stats: LogStats,
    pub key_state: KeyState,
    pub rx: mpsc::Receiver<AppEvent>,
    _tx: mpsc::Sender<AppEvent>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        
        // Arti Task
        let tx_arti = tx.clone();
        tokio::spawn(async move {
            let mut monitor = ArtiMonitor::new();
            loop {
                let status = monitor.check();
                let _ = tx_arti.send(AppEvent::ArtiUpdate(status)).await;
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        });

        // SOCKS Task
        let tx_socks = tx.clone();
        tokio::spawn(async move {
            loop {
                let active = test_socks_proxy().await;
                let _ = tx_socks.send(AppEvent::SocksUpdate(active)).await;
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });

        // Logs Task
        let tx_logs = tx.clone();
        let log_dir = std::path::PathBuf::from("/Users/rds/antigravity/umbra/var/log/");
        tail_file(log_dir, tx_logs);

        // Onion Task
        let tx_onion = tx.clone();
        tokio::spawn(async move {
            let config_path = std::path::PathBuf::from("/Users/rds/antigravity/umbra/arti.toml");
            let onion_dir = std::path::PathBuf::from("/Users/rds/antigravity/umbra/var/lib/arti/onion_services");
            let scanner = OnionScanner::new(onion_dir);
            
            loop {
                let services = read_onion_services(config_path.clone());
                let status = scanner.scan(&services);
                let _ = tx_onion.send(AppEvent::OnionUpdate(status)).await;
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });

        // Deps Task
        let tx_deps = tx.clone();
        tokio::spawn(async move {
            loop {
                let apps = scan_dependent_apps();
                let _ = tx_deps.send(AppEvent::DepsUpdate(apps)).await;
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });

        Self {
            should_quit: false,
            arti_status: ArtiStatus::default(),
            socks_active: false,
            logs: Vec::new(),
            log_scroll: 0,
            log_auto_scroll: true,
            onion_services: Vec::new(),
            dependent_apps: Vec::new(),
            confirm_action: None,
            notification: None,
            log_stats: LogStats::default(),
            key_state: KeyState::Normal,
            rx,
            _tx: tx,
        }
    }
    
    // ... on_key ...


    fn on_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        if let Some(action) = self.confirm_action {
             match code {
                 KeyCode::Char('y') | KeyCode::Enter => {
                     action.execute();
                     self.confirm_action = None;
                 }
                 KeyCode::Char('n') | KeyCode::Esc => {
                     self.confirm_action = None;
                 }
                 _ => {}
             }
             return;
        }

        match self.key_state {
            KeyState::Normal => {
                match code {
                    KeyCode::Char('q') => self.should_quit = true,
                    KeyCode::Char('s') => {
                        if self.arti_status.running {
                             self.notification = Some(("ARTI IS ALREADY RUNNING".to_string(), std::time::Instant::now()));
                        } else {
                             self.confirm_action = Some(Action::Start);
                        }
                    }
                    KeyCode::Char('x') => self.confirm_action = Some(Action::Stop),
                    KeyCode::Char('r') => self.confirm_action = Some(Action::Restart),
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.log_scroll = self.log_scroll.saturating_add(1);
                        self.log_auto_scroll = false;
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.log_scroll = self.log_scroll.saturating_sub(1);
                        self.log_auto_scroll = false;
                    }
                    KeyCode::Char('G') => {
                        self.log_auto_scroll = true;
                        // Scroll will be updated in UI render or update loop
                    }
                    KeyCode::Char('g') => {
                        self.key_state = KeyState::WaitingForG(std::time::Instant::now());
                    }
                    KeyCode::Char('d') if modifiers.contains(KeyModifiers::CONTROL) => {
                        self.log_scroll = self.log_scroll.saturating_add(10);
                        self.log_auto_scroll = false;
                    }
                    KeyCode::Char('u') if modifiers.contains(KeyModifiers::CONTROL) => {
                         self.log_scroll = self.log_scroll.saturating_sub(10);
                         self.log_auto_scroll = false;
                    }
                    _ => {}
                }
            }
            KeyState::WaitingForG(start) => {
                // Check timeout (500ms)
                if start.elapsed() > Duration::from_millis(500) {
                    self.key_state = KeyState::Normal;
                }
                
                match code {
                    KeyCode::Char('g') => {
                        // gg -> Go to top
                        self.log_scroll = 0;
                        self.log_auto_scroll = false;
                        self.key_state = KeyState::Normal;
                    }
                    _ => {
                        self.key_state = KeyState::Normal;
                    }
                }
            }
        }
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let mut interval = tokio::time::interval(Duration::from_millis(100));

        loop {
            // Auto-scroll logic (before draw)
            if self.log_auto_scroll && !self.logs.is_empty() {
                self.log_scroll = self.logs.len().saturating_sub(1);
            }
            
            // Draw UI
            terminal.draw(|f| ui(f, self))?;

            // Handle Events
            tokio::select! {
                _ = interval.tick() => {
                    if let Some((_, time)) = self.notification {
                        if time.elapsed() > Duration::from_secs(2) {
                            self.notification = None;
                        }
                    }
                    // Check timeout for 'g' state
                     if let KeyState::WaitingForG(start) = self.key_state {
                        if start.elapsed() > Duration::from_millis(500) {
                            self.key_state = KeyState::Normal;
                        }
                    }
                }
                Some(event) = self.rx.recv() => {
                    match event {
                        AppEvent::ArtiUpdate(status) => self.arti_status = status,
                        AppEvent::SocksUpdate(active) => self.socks_active = active,
                        AppEvent::LogUpdate(mut new_lines) => {
                            for line in &new_lines {
                                match line.level {
                                    LogLevel::Error => self.log_stats.error_count += 1,
                                    LogLevel::Warn => self.log_stats.warn_count += 1,
                                    _ => {}
                                }
                                if line.scrubbed {
                                    self.log_stats.scrub_count += 1;
                                }
                            }
                            self.logs.append(&mut new_lines);
                            // Cap buffer optionally? 2000 lines
                            if self.logs.len() > 2000 {
                                let remove = self.logs.len() - 2000;
                                self.logs.drain(0..remove);
                                if !self.log_auto_scroll {
                                    self.log_scroll = self.log_scroll.saturating_sub(remove);
                                }
                            }
                        }
                        AppEvent::OnionUpdate(s) => self.onion_services = s,
                        AppEvent::DepsUpdate(d) => self.dependent_apps = d,
                    }
                }
                event = async { event::poll(Duration::from_millis(10)) } => {
                     if let Ok(true) = event {
                        if let Ok(Event::Key(key)) = event::read() {
                             self.on_key(key.code, key.modifiers);
                        }
                     }
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }
}
