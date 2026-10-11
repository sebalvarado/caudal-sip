use std::io;
use std::time::Duration;

use caudal_risk::Light;
use caudal_south_api::Op;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line as TextLine, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row as TableRow, Table, Wrap};
use ratatui::{DefaultTerminal, Frame};

use crate::commands::{self, Line, Tone, clock, name_of, state_label, state_tone};
use crate::sandbox::{Sandbox, money};
use crate::scenarios::DISCLAIMER;

const ACCENT: Color = Color::Rgb(255, 176, 0);

fn tone_style(tone: Tone) -> Style {
    match tone {
        Tone::Plain => Style::default(),
        Tone::Heading => Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        Tone::Good => Style::default().fg(Color::Green),
        Tone::Bad => Style::default().fg(Color::Red),
        Tone::Warn => Style::default().fg(Color::Yellow),
        Tone::Dim => Style::default().fg(Color::DarkGray),
        Tone::Xml => Style::default().fg(Color::Cyan),
    }
}

fn light_color(light: Light) -> Color {
    match light {
        Light::Green => Color::Green,
        Light::Yellow => Color::Yellow,
        Light::Orange => Color::Rgb(255, 140, 0),
        Light::Red => Color::Red,
    }
}

struct App {
    sandbox: Sandbox,
    input: String,
    history: Vec<String>,
    recall: Option<usize>,
    output: Vec<Line>,
    scroll: u16,
    quit: bool,
}

impl App {
    fn new() -> Self {
        let mut output = vec![
            Line { tone: Tone::Heading, text: "Caudal: the whole network in one process".into() },
            Line {
                tone: Tone::Plain,
                text: "SIP, red alfa, red beta and four fictional institutions, with signed ISO 20022 between them."
                    .into(),
            },
            Line {
                tone: Tone::Plain,
                text: "Try:  pay banco-alfa llave-01 25.40   then   settle   then   show last xml".into(),
            },
            Line {
                tone: Tone::Dim,
                text: "help lists every command · PgUp/PgDn scroll · ↑/↓ history · Esc quits".into(),
            },
        ];
        output.extend(commands::help());
        Self {
            sandbox: Sandbox::new(),
            input: String::new(),
            history: Vec::new(),
            recall: None,
            output,
            scroll: 0,
            quit: false,
        }
    }

    fn submit(&mut self) {
        let line = std::mem::take(&mut self.input);
        if line.trim().is_empty() {
            return;
        }
        self.history.push(line.clone());
        self.recall = None;
        self.output = vec![Line { tone: Tone::Dim, text: format!("> {line}") }];
        match commands::parse(&line) {
            Ok(command) => {
                let out = commands::execute(&mut self.sandbox, command);
                self.quit = out.quit;
                self.output.extend(out.lines);
            }
            Err(e) => self.output.push(Line { tone: Tone::Bad, text: e }),
        }
        self.scroll = 0;
    }

    fn key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        match code {
            KeyCode::Esc => self.quit = true,
            KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => self.quit = true,
            KeyCode::Enter => self.submit(),
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::PageDown => self.scroll = self.scroll.saturating_add(10),
            KeyCode::PageUp => self.scroll = self.scroll.saturating_sub(10),
            KeyCode::Up if !self.history.is_empty() => {
                let i = self.recall.map_or(self.history.len() - 1, |i| i.saturating_sub(1));
                self.recall = Some(i);
                self.input = self.history[i].clone();
            }
            KeyCode::Down => match self.recall {
                Some(i) if i + 1 < self.history.len() => {
                    self.recall = Some(i + 1);
                    self.input = self.history[i + 1].clone();
                }
                _ => {
                    self.recall = None;
                    self.input.clear();
                }
            },
            _ => {}
        }
    }
}

pub fn run() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = event_loop(&mut terminal);
    ratatui::restore();
    result
}

fn event_loop(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();
    while !app.quit {
        terminal.draw(|f| draw(f, &app))?;
        if event::poll(Duration::from_millis(250))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            app.key(key.code, key.modifiers);
        }
    }
    Ok(())
}

fn draw(f: &mut Frame, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Percentage(55), Constraint::Min(6), Constraint::Length(3)])
        .split(f.area());
    header(f, rows[0], app);
    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(56), Constraint::Percentage(44)])
        .split(rows[1]);
    blotter(f, top[0], app);
    let side = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Length(7), Constraint::Min(5)])
        .split(top[1]);
    headroom(f, side[0], app);
    positions(f, side[1], app);
    stats(f, side[2], app);
    let lines: Vec<TextLine> =
        app.output.iter().map(|l| TextLine::styled(l.text.clone(), tone_style(l.tone))).collect();
    f.render_widget(
        Paragraph::new(lines).block(panel("Output")).wrap(Wrap { trim: false }).scroll((app.scroll, 0)),
        rows[2],
    );
    let prompt = TextLine::from(vec![Span::styled("> ", Style::default().fg(ACCENT)), Span::raw(app.input.as_str())]);
    f.render_widget(Paragraph::new(prompt).block(panel("Command")), rows[3]);
    f.set_cursor_position((rows[3].x + 3 + app.input.chars().count() as u16, rows[3].y + 1));
}

fn panel(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(format!(" {title} "), Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)))
}

fn header(f: &mut Frame, area: Rect, app: &App) {
    let line = TextLine::from(vec![
        Span::styled(" CAUDAL ", Style::default().fg(Color::Black).bg(ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled(format!("  RPI EC sandbox  {}  ", clock(app.sandbox.now())), Style::default().fg(ACCENT)),
        Span::styled(DISCLAIMER, Style::default().fg(Color::DarkGray)),
    ]);
    f.render_widget(Paragraph::new(line), area);
}

fn blotter(f: &mut Frame, area: Rect, app: &App) {
    let rows = app.sandbox.network().rows();
    let visible = area.height.saturating_sub(3) as usize;
    let body: Vec<TableRow> = rows
        .iter()
        .rev()
        .take(visible)
        .map(|r| {
            TableRow::new(vec![
                Cell::from(clock(r.created)),
                Cell::from(r.id.as_str()[r.id.as_str().len() - 6..].to_string()),
                Cell::from(format!("{} → {}", name_of(r.from), name_of(r.to))),
                Cell::from(format!("{:>10}", r.amount.to_string())),
                Cell::from(state_label(r)).style(tone_style(state_tone(r))),
            ])
        })
        .collect();
    let widths = [
        Constraint::Length(12),
        Constraint::Length(6),
        Constraint::Min(26),
        Constraint::Length(10),
        Constraint::Length(22),
    ];
    let title = format!("Blotter · {} payments", rows.len());
    let head = TableRow::new(["time", "id", "from → to", "USD", "state"]).style(Style::default().fg(Color::DarkGray));
    f.render_widget(Table::new(body, widths).header(head).block(panel(&title)), area);
}

fn headroom(f: &mut Frame, area: Rect, app: &App) {
    let body: Vec<TableRow> = app
        .sandbox
        .institutions()
        .into_iter()
        .map(|v| {
            let used = (v.used().max(0) as f64 / v.ceiling.cents().max(1) as f64).min(1.0);
            let filled = (used * 10.0).round() as usize;
            let bar = format!("{}{}", "█".repeat(filled), "·".repeat(10 - filled));
            TableRow::new(vec![
                Cell::from(v.id),
                Cell::from(format!("{:>13}", money(i128::from(v.ceiling.cents())))),
                Cell::from(format!("{:>13}", money(v.available))),
                Cell::from(bar).style(Style::default().fg(light_color(v.light))),
                Cell::from(v.light.label()).style(Style::default().fg(light_color(v.light))),
            ])
        })
        .collect();
    let widths = [
        Constraint::Length(13),
        Constraint::Length(13),
        Constraint::Length(13),
        Constraint::Length(10),
        Constraint::Length(8),
    ];
    let head = TableRow::new(["institution", "ceiling", "available", "used", "light"])
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(Table::new(body, widths).header(head).block(panel("Headroom")), area);
}

fn positions(f: &mut Frame, area: Rect, app: &App) {
    let body: Vec<TableRow> = app
        .sandbox
        .institutions()
        .into_iter()
        .map(|v| {
            let style = Style::default().fg(if v.net < 0 { Color::Yellow } else { Color::Green });
            TableRow::new(vec![
                Cell::from(v.id),
                Cell::from(v.red),
                Cell::from(format!("{:>14}", money(v.net))).style(style),
            ])
        })
        .collect();
    let last = app
        .sandbox
        .network()
        .settlements()
        .last()
        .map_or("no settlement yet".into(), |s| format!("settled {}", clock(s.at)));
    let widths = [Constraint::Length(13), Constraint::Length(9), Constraint::Length(14)];
    let head = TableRow::new(["institution", "red", "net"]).style(Style::default().fg(Color::DarkGray));
    f.render_widget(Table::new(body, widths).header(head).block(panel(&format!("Malla · {last}"))), area);
}

fn stats(f: &mut Frame, area: Rect, app: &App) {
    let s = app.sandbox.network().stats();
    let us = |d: Duration| format!("{:.1}µs", d.as_nanos() as f64 / 1_000.0);
    let mut body: Vec<TableRow> = Op::ALL
        .into_iter()
        .filter_map(|op| {
            s.summary(op).map(|x| {
                TableRow::new(vec![
                    Cell::from(op.label()),
                    Cell::from(format!("{:>7}", x.count)),
                    Cell::from(format!("{:>9}", us(x.p50))),
                    Cell::from(format!("{:>9}", us(x.p95))),
                    Cell::from(format!("{:>9}", us(x.p99))),
                    Cell::from(format!("{:>9.0}/s", x.per_second())),
                ])
            })
        })
        .collect();
    if let Some(x) = s.reserve_to_post() {
        let ms = |d: Duration| format!("{:.0}ms", d.as_secs_f64() * 1_000.0);
        body.push(TableRow::new(vec![
            Cell::from("reserve→post"),
            Cell::from(format!("{:>7}", x.count)),
            Cell::from(format!("{:>9}", ms(x.p50))),
            Cell::from(format!("{:>9}", ms(x.p95))),
            Cell::from(format!("{:>9}", ms(x.p99))),
            Cell::from("simulated").style(Style::default().fg(Color::DarkGray)),
        ]));
    }
    let widths = [
        Constraint::Length(13),
        Constraint::Length(8),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(12),
    ];
    let head =
        TableRow::new(["op", "n", "p50", "p95", "p99", "throughput"]).style(Style::default().fg(Color::DarkGray));
    f.render_widget(Table::new(body, widths).header(head).block(panel("Compute per hop · a floor")), area);
}
