use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, symbols::border, text::{Line, Span, Text}, widgets::{Block, Paragraph, Widget, Wrap}, Frame};

use crate::{alu::ALU, bus::Bus, config::{OPCODE_SIZE, WORD_SIZE}, control::controller::Controller, link::Link, memory::{memory::RAM, register::{RORegister, RWRegister}}, pc::ProgramCounter, BinaryDisplay};

impl Widget for &ProgramCounter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Program Counter ");
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let bindata = self.address.to_bin_string();
        let decdata = self.address.iter().fold(0u32, |acc, bit| (acc << 1) | u32::from(bit));
        let mut widgetlines = Vec::new();
        widgetlines.push(Line::from(vec![Span::styled(format!("{:X} | {} | {}", decdata, bindata.clone(), decdata), Style::default().fg(Color::Yellow))]));
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
            let decdata = self.data.iter().fold(0u32, |acc, bit| (acc << 1) | u32::from(bit));
            let mut widgetlines = Vec::new();
            widgetlines.push(Line::from(vec![Span::styled(format!("{:X} | {} | {}", decdata, bindata.clone(), decdata), Style::default().fg(Color::Yellow))]));
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
        let decdata = self.data.iter().fold(0u32, |acc, bit| (acc << 1) | u32::from(bit));
        let mut widgetlines = Vec::new();
        widgetlines.push(Line::from(vec![Span::styled(format!("{:X} | {} | {}", decdata, bindata.clone(), decdata), Style::default().fg(Color::Yellow))]));
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

impl Widget for &RAM {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" RAM Inspector ");
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let mut lines = Vec::new();
        for addr in 0..self.memory.len() {
            let line_color = if addr < (1 << (WORD_SIZE - OPCODE_SIZE)) {
                Color::Yellow
            } else {
                Color::Gray
            };
            lines.push(Line::from(vec![
                Span::styled(format!("{:01$b}|", addr, WORD_SIZE), Style::default().fg(Color::White)),
                Span::styled(self.memory[addr].to_bin_string(), Style::default().fg(line_color)),
            ]));
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &Bus {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("W Bus");

        // Create the block with the title centered horizontally
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let bindata = self.data.to_bin_string();
        let decdata = self.data.iter().fold(0u32, |acc, bit| (acc << 1) | u32::from(bit));
        let mut widgetlines = Vec::new();
        
        // Add the formatted lines for bus data
        widgetlines.push(Line::from(vec![Span::styled(
            format!("{:X} | {} | {}", decdata, bindata.clone(), decdata),
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
impl<'a> Widget for &'a Controller<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Controller Sequencer ");
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        // Render the block (bordered title section)
        Paragraph::new(vec![Line::from("")]) // Empty body text for the block
            .block(block)
            .render(area, buf);

        // Define the remaining area for controls (subtracting space for the block)
        let control_area = Rect {
            x: area.x,
            y: area.y + 3, // Leave some space for the block
            width: area.width,
            height: area.height - 3, // Adjust height accordingly
        };

        // Get the controls and calculate the percentage for each one
        let controls = self.control_links.keys().collect::<Vec<_>>();
        let num_controls = controls.len() as u16;
        let percentage_per_control = 100 / num_controls;

        // Dynamically generate constraints based on the number of controls
        let constraints = (0..num_controls)
            .map(|_| Constraint::Percentage(percentage_per_control))
            .collect::<Vec<_>>();

        // Define the layout for controls within the remaining area
        let link_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(control_area);

        // Render controls in the layout
        for (i, key) in controls.iter().enumerate() {
            if let Some(link) = self.control_links.get(*key) {
                let link = link.borrow(); // Borrow the Link instance
                let color = if link.get_state() {
                    Color::Yellow
                } else {
                    Color::White
                };

                let body_text = Text::from(vec![
                    Line::from(Span::styled(
                        format!("{}", link.control),
                        Style::default().fg(color),
                    )),
                ]);

                // Render the paragraph inside the layout cell
                Paragraph::new(body_text).render(link_layout[i], buf);
            }
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