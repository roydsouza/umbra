use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};
use crate::app::AppEvent;

#[derive(Debug, Clone)]
pub struct LogLine {
    pub content: String,
    pub level: LogLevel,
    pub scrubbed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
    Unknown,
}

pub fn parse_level(line: &str) -> LogLevel {
    if line.contains("INFO") { LogLevel::Info }
    else if line.contains("WARN") { LogLevel::Warn }
    else if line.contains("ERROR") { LogLevel::Error }
    else if line.contains("DEBUG") { LogLevel::Debug }
    else { LogLevel::Unknown }
}

use regex::Regex;

pub fn tail_file(path: PathBuf, tx: tokio::sync::mpsc::Sender<AppEvent>) {
    tokio::spawn(async move {
        // Simple IPv4 regex
        let ip_regex = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").expect("Invalid regex");
        let mut offset = 0u64;
        
        // Initial Seek Logic (Set starting offset)
        if let Ok(m) = std::fs::metadata(&path) {
            let len = m.len();
            if len > 50_000 {
                offset = len - 50_000;
            }
        }

        loop {
            // Check file metadata
            let len = match std::fs::metadata(&path) {
                Ok(m) => m.len(),
                Err(_) => {
                    // File might not exist yet or permission error
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    continue;
                }
            };

            // Detect Truncation or Rotation
            if len < offset {
                offset = 0;
            }

            // Read new data if available
            if len > offset {
                 if let Ok(mut file) = File::open(&path) {
                     if file.seek(SeekFrom::Start(offset)).is_ok() {
                         let mut reader = BufReader::new(file);
                         let mut buffer = String::new();
                         let mut new_lines = Vec::new();
                         let mut bytes_read = 0;
                         
                         while let Ok(n) = reader.read_line(&mut buffer) {
                             if n == 0 { break; }
                             bytes_read += n;
                             
                             // Scrub IPs (skip if localhost loopback 127.0.0.1?)
                             // Current regex catches 127.0.0.1. 
                             // Prompt said "Scrubbing I will implement basic regex-based scrubbing".
                             // Let's just scrub all for safety as requested.
                             let scrubbed_content = ip_regex.replace_all(buffer.trim_end(), "[scrubbed]");
                             let is_scrubbed = match scrubbed_content {
                                 std::borrow::Cow::Borrowed(_) => false,
                                 std::borrow::Cow::Owned(_) => true,
                             };
                             
                             new_lines.push(LogLine { 
                                 level: parse_level(&buffer),
                                 content: scrubbed_content.to_string(),
                                 scrubbed: is_scrubbed,
                             });
                             buffer.clear();
                         }
                         
                         if !new_lines.is_empty() {
                             if tx.send(AppEvent::LogUpdate(new_lines)).await.is_err() {
                                 break; // App closed channel
                             }
                             offset += bytes_read as u64;
                         }
                     }
                 }
            }
            
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });
}
