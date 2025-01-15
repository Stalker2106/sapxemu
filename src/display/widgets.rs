use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, symbols::border, text::{Line, Span, Text}, widgets::{Block, Paragraph, Widget, Wrap}, Frame};

use crate::{alu::ALU, bitvecutils::{bitvec_to_usize, BinaryDisplay}, bus::Bus, clock::Clock, config::{OPCODE_SIZE, WORD_SIZE}, control::{control::ControlLine, controller::Controller}, link::Link, memory::{memory::RAM, register::{RORegister, RWRegister}}, pc::ProgramCounter};

impl Widget for &ProgramCounter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Program Counter ");
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let bindata = self.address.to_bin_string();
        let decdata = bitvec_to_usize(&self.address);
        let mut widgetlines = Vec::new();
        widgetlines.push(Line::from(vec![Span::styled(format!("0x{:X} | {} | {}", decdata, bindata.clone(), decdata), Style::default().fg(Color::Yellow))]));
        widgetlines.push(Line::from(vec![Span::styled(bindata.replace('0', "◯").replace('1', "●"), Style::default().fg(Color::Yellow))]));

        Paragraph::new(widgetlines)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &RORegister {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(format!(" {} ", self.name));
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

            let bindata = self.data.to_bin_string();
            let decdata = bitvec_to_usize(&self.data);
            let mut widgetlines = Vec::new();
            widgetlines.push(Line::from(vec![Span::styled(format!("0x{:X} | {} | {}", decdata, bindata.clone(), decdata), Style::default().fg(Color::Yellow))]));
            widgetlines.push(Line::from(vec![Span::styled(bindata.replace('0', "◯").replace('1', "●"), Style::default().fg(Color::Yellow))]));
    

        Paragraph::new(widgetlines)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &RWRegister {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(format!(" {} ", self.name));
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let bindata = self.data.to_bin_string();
        let decdata = bitvec_to_usize(&self.data);
        let mut widgetlines = Vec::new();
        widgetlines.push(Line::from(vec![Span::styled(format!("0x{:X} | {} | {}", decdata, bindata.clone(), decdata), Style::default().fg(Color::Yellow))]));
        widgetlines.push(Line::from(vec![Span::styled(bindata.replace('0', "◯").replace('1', "●"), Style::default().fg(Color::Yellow))]));

        Paragraph::new(widgetlines)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &ALU {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" ALU ");
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let body_text = Text::from(vec![Line::from(vec![
            Span::raw(""), // Plain text span
        ])]);

        Paragraph::new(body_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &Clock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Clock ");
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let body_text = Text::from(vec![Line::from(vec![
            Span::raw("TICK"), // Plain text span
        ])]);

        Paragraph::new(body_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &RAM {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" RAM ");
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let body_text = Text::from(vec![Line::from(vec![
            Span::raw(""), // Plain text span
        ])]);

        Paragraph::new(body_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

pub fn render_ram_inspector(frame: &mut Frame, ram: &RAM, area: Rect) {
    let title = Line::from(" RAM Inspector ");
    let block = Block::bordered()
        .title(title.centered())
        .border_set(border::THICK);

    let mut lines = Vec::new();
    lines.push(Line::from(vec![
        Span::styled("ADDRESS|DATA", Style::default().fg(Color::White)),
    ]));
    for addr in 0..ram.memory.len() {
        let addr_color = if addr < (1 << (WORD_SIZE - OPCODE_SIZE)) {
            if addr == bitvec_to_usize(&ram.mar.borrow().read()) {
                Color::Red
            } else {
                Color::White
            }
        } else {
            Color::Gray
        };
        let data_color = if addr < (1 << (WORD_SIZE - OPCODE_SIZE)) {
            Color::Yellow
        } else {
            Color::Gray
        };
        lines.push(Line::from(vec![
            Span::styled(format!("{:01$b}|", addr, WORD_SIZE), Style::default().fg(addr_color)),
            Span::styled(ram.memory[addr].to_bin_string(), Style::default().fg(data_color)),
        ]));
    }

    frame.render_widget(
 Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .block(block),
        area
    );
}

impl Widget for &Bus {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("W Bus");

        // Create the block with the title centered horizontally
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let bindata = self.data.to_bin_string();
        let decdata = bitvec_to_usize(&self.data);
        let mut widgetlines = Vec::new();
        
        // Add the formatted lines for bus data
        widgetlines.push(Line::from(vec![Span::styled(
            format!("0x{:X} | {} | {}", decdata, bindata.clone(), decdata),
            Style::default().fg(Color::Yellow),
        )]));
        widgetlines.push(Line::from(vec![Span::styled(
            bindata.replace('0', "◯").replace('1', "●"),
            Style::default().fg(Color::Yellow),
        )]));

        // Calculate the number of blank lines to add above and below the content
        let content_height = widgetlines.len() as u16;
        let blank_lines_top = (area.height - content_height) / 2;
        let blank_lines_bottom = area.height - content_height - blank_lines_top;

        // Create the lines with blank lines at the top and bottom
        let mut final_lines = vec![];

        // Add blank lines at the top
        for _ in 0..blank_lines_top {
            final_lines.push(Line::from(""));
        }

        // Add the actual content (the bus data lines)
        final_lines.extend(widgetlines);

        // Add blank lines at the bottom
        for _ in 0..blank_lines_bottom {
            final_lines.push(Line::from(""));
        }

        // Create the paragraph with the final lines
        let paragraph = Paragraph::new(final_lines)
            .block(block);

        // Render the paragraph in the original area
        paragraph.render(area, buf);
    }
}

impl Widget for &Controller {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Controller Sequencer ");
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        // Render the block (bordered title section)
        Paragraph::new(vec![Line::from("")]) // Empty body text for the block
            .block(block)
            .render(area, buf);
    }
}

pub fn render_all_links(frame: &mut Frame, control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>, area: Rect) {
    let controls = control_links.keys().collect::<Vec<_>>();
    let num_controls = controls.len() as u16;

    // Guard against division by zero
    if num_controls == 0 {
        return;
    }

    // Calculate the percentage for each control
    let percentage_per_control = 100 / num_controls;

    // Dynamically generate constraints for layout
    let constraints = (0..num_controls)
        .map(|_| Constraint::Percentage(percentage_per_control))
        .collect::<Vec<_>>();

    // Split the available space horizontally for links
    let link_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .horizontal_margin(2) // Add horizontal margin for centering
        .split(area);

    // Render each control in its respective layout cell
    for (i, key) in controls.iter().enumerate() {
        if let Some(link) = control_links.get(*key) {
            let link = link.borrow(); // Borrow the Link instance
            let color = if link.get_state() {
                Color::Yellow
            } else {
                Color::White
            };

            let body_text = Text::from(vec![
                Line::from(Span::styled(format!("{}", link.control), Style::default().fg(color))),
                Line::from(Span::styled("|", Style::default().fg(color))),
            ]);

            // Render the paragraph inside the layout cell
            frame.render_widget(
                Paragraph::new(body_text).alignment(Alignment::Center), // Center the text within the widget
                link_layout[i],
            );
        }
    }
}

pub fn render_h_link(frame: &mut Frame, link: &Link, left: bool, area: Rect) {
    let display_text = if left {
        format!("{}🭹", link.control)
    } else {
        format!("🭹{}", link.control)
    };

    // Determine text color based on `link.get_state()`
    let text_color = if link.get_state() {
        Color::Yellow
    } else {
        Color::White
    };

    // Create the styled text
    let body_text = Text::from(vec![Line::from(vec![
        Span::styled(display_text, Style::default().fg(text_color)),
    ])]);

    // Determine alignment based on the `left` flag
    let alignment = if left {
        Alignment::Right
    } else {
        Alignment::Left
    };

    let centered_area = Rect {
        x: area.x,
        y: area.y + (area.height.saturating_sub(1) / 2), // Center vertically
        width: area.width,
        height: 1, // Only one line of height for the text
    };

    // Render the paragraph
    frame.render_widget(
        Paragraph::new(body_text).alignment(alignment),
        centered_area,
    );
}

pub enum BusConnection {
    Left,
    Right,
    Both
}

pub fn render_bus_connection(frame: &mut Frame, connection: BusConnection, state: bool, area: Rect) {
    // Ensure at least 1 line in the center
    let center_y = area.height / 2; // Single center line

    // Determine the pattern based on connection type
    let pattern = match connection {
        BusConnection::Left => format!("🭮{}", "█".repeat((area.width - 1) as usize)),
        BusConnection::Right => format!("{}🭬", "█".repeat((area.width - 1) as usize)),
        BusConnection::Both => format!("🭮{}🭬", "█".repeat((area.width - 2) as usize)),
    };

    // Generate the lines with only the center row rendered
    let lines: Vec<Line> = (0..area.height)
        .map(|i| {
            if i == center_y {
                // Check the state and apply yellow color if state is true
                let style = if state {
                    Style::default().fg(Color::Yellow) // Yellow color
                } else {
                    Style::default() // Default style (no color change)
                };

                // Draw the pattern at the center with the determined style
                Line::from(Span::styled(&pattern, style))
            } else {
                // Empty row for all other lines
                Line::from("")
            }
        })
        .collect();

    // Create the text and render the widget
    let text = Text::from(lines);
    frame.render_widget(Paragraph::new(text), area);
}

pub enum ICConnection {
    Up,
    Down,
    Both
}

pub fn render_ic_connection(frame: &mut Frame, connection: ICConnection, state: bool, area: Rect) {
    // Define the layout constraints (20% left, 70% center, 10% right)
    let constraints = vec![
        Constraint::Percentage(20), // 20% for the left margin
        Constraint::Percentage(70), // 70% for the main content area
        Constraint::Percentage(10), // 10% for the right margin
    ];

    // Split the area based on the constraints
    let layout = Layout::default()
        .direction(Direction::Horizontal) // Horizontal layout (left to right)
        .constraints(constraints)         // Apply the defined constraints
        .split(area);                     // Split the given area

    // Determine the pattern based on connection type
    let pattern = match connection {
        ICConnection::Up => vec!["██", "██"],
        ICConnection::Down => vec!["██", "██"],
        ICConnection::Both => vec!["██", "██"],
    };

    // Choose the color based on the state
    let color = if state {
        Color::Yellow
    } else {
        Color::Gray
    };

    // Convert the pattern into styled lines
    let lines: Vec<Line> = pattern
        .into_iter()
        .map(|symbol| Line::from(Span::styled(symbol, Style::default().fg(color))))
        .collect();

    // Create the text and render the widget in the 70% area
    let text = Text::from(lines);
    frame.render_widget(Paragraph::new(text).alignment(Alignment::Center), layout[1]); // layout[1] corresponds to the 70% area
}