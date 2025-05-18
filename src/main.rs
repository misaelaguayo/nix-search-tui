use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    process::{Command, Stdio},
};

use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::Color,
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    event_stream: EventStream,
    search: String,
    highlighted_index: Option<usize>,
    results: Vec<SearchResult>,
    show_popup: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlakeResolved {
    r#type: String,
    owner: String,
    repo: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    r#type: String,
    package_pname: String,
    package_attr_name: String,
    package_attr_set: String,
    package_outputs: Vec<String>,
    package_description: String,
    package_programs: Vec<String>,
    package_homepage: Vec<String>,
    package_pversion: String,
    package_platforms: Vec<String>,
    package_position: String,
    package_license: Vec<HashMap<String, String>>,
    flake_name: String,
    flake_description: String,
    flake_resolved: FlakeResolved,
}

pub fn search_nix(search: &str) -> Result<Vec<SearchResult>> {
    let mut command = Command::new("nix-search");
    command.args(["--json", search]);

    command.arg("search").arg(search);
    command.stdout(Stdio::piped());

    let output = command.output().expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let res: Vec<SearchResult> = stdout
            .lines()
            .map(|line| return serde_json::from_str(line).unwrap())
            .collect();

        let filtered_results: Vec<SearchResult> = res
            .into_iter()
            .unique_by(|r| r.package_attr_name.clone())
            .collect();

        Ok(filtered_results)
    } else {
        Err(color_eyre::eyre::eyre!("Command failed to execute"))
    }
}

#[test]
fn search_nix_should_return_search_results() {
    let search = "python";
    let result = search_nix(search);
    assert!(result.is_ok());
    let results = result.unwrap();
    assert!(!results.is_empty());
}

impl App {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events().await?;
        }
        Ok(())
    }

    fn search_nix(&self) -> Vec<SearchResult> {
        search_nix(&self.search).unwrap()
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]);
        let [search_area, results_area] = vertical.areas(frame.area());
        let search = Paragraph::new(self.search.as_str()).block(Block::bordered().title("Search"));
        frame.render_widget(search, search_area);

        let search_results: Vec<ListItem> = self
            .results
            .iter()
            .enumerate()
            .map(|(i, r)| {
                if self.highlighted_index == Some(i) {
                    return ListItem::new(Line::styled(
                        &r.package_attr_name,
                        ratatui::style::Style::default().bg(Color::Blue),
                    ));
                }
                ListItem::new(Span::raw(&r.package_attr_name))
            })
            .collect();
        let search_results = List::new(search_results).block(Block::bordered().title("Results"));
        frame.render_widget(search_results, results_area);

        if self.show_popup {
            let highlighted_result = self
                .highlighted_index
                .and_then(|i| self.results.get(i))
                .unwrap_or(&self.results[0])
                .clone();
            let result_details: Vec<ListItem> = vec![
                ListItem::new(Span::raw(format!(
                    "Package: {}",
                    highlighted_result.package_attr_name
                ))),
                ListItem::new(Span::raw(format!(
                    "Description: {}",
                    highlighted_result.package_description
                ))),
                ListItem::new(Span::raw(format!(
                    "Version: {}",
                    highlighted_result.package_pversion
                ))),
            ];
            let popup_area = Self::popup_area(frame.area(), 60, 20);
            let popup = List::new(result_details)
                .block(Block::bordered().title(highlighted_result.package_attr_name))
                .highlight_style(ratatui::style::Style::default().bg(Color::Blue));
            frame.render_widget(popup, popup_area);
        }
    }

    /// Reads the crossterm events and updates the state of [`App`].
    async fn handle_crossterm_events(&mut self) -> Result<()> {
        tokio::select! {
            event = self.event_stream.next().fuse() => {
                match event {
                    Some(Ok(evt)) => {
                        match evt {
                            Event::Key(key)
                                if key.kind == KeyEventKind::Press
                                    => self.on_key_event(key),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                match self.show_popup {
                    true => {
                        self.show_popup = false;
                    }
                    false => {
                        self.quit();
                    }
                }
            }
            // Add other key handlers here.
            (_, KeyCode::Char(c)) => {
                self.search.push(c);
                self.highlighted_index = None;
            }
            (_, KeyCode::Backspace) => {
                self.search.pop();
                self.highlighted_index = None;
            }
            (_, KeyCode::Enter) => match self.highlighted_index {
                Some(_) => {
                    self.show_popup = true;
                }
                None => {
                    self.results = self.search_nix();
                    self.highlighted_index = Some(0);
                }
            },
            (_, KeyCode::Down) => {
                if let Some(index) = self.highlighted_index {
                    if index < self.results.len() - 1 {
                        self.highlighted_index = Some(index + 1);
                    }
                }
            }
            (_, KeyCode::Up) => {
                if let Some(index) = self.highlighted_index {
                    if index > 0 {
                        self.highlighted_index = Some(index - 1);
                    }
                }
            }
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }
}
