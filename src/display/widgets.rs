use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Style}, symbols::border, text::{Line, Span, Text}, widgets::{Block, Paragraph, Widget, Wrap}};

use crate::{alu::ALU, bus::Bus, config::WORD_SIZE, link::Link, memory::{memory::RAM, register::{RORegister, RWRegister}}, pc::ProgramCounter, BinaryDisplay};

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
        widgetlines.push(Line::from(vec![Span::styled(bindata.replace('0', "○").replace('1', "●"), Style::default().fg(Color::Yellow))]));

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
            widgetlines.push(Line::from(vec![Span::styled(bindata.replace('0', "○").replace('1', "●"), Style::default().fg(Color::Yellow))]));
    

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
        widgetlines.push(Line::from(vec![Span::styled(bindata.replace('0', "○").replace('1', "●"), Style::default().fg(Color::Yellow))]));

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
            lines.push(Line::from(vec![
                Span::styled(format!("{:01$b}|", addr, WORD_SIZE), Style::default().fg(Color::White)),
                Span::styled(self.memory[addr].to_bin_string(), Style::default().fg(Color::Yellow)),
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
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let bindata = self.data.to_bin_string();
        let decdata = self.data.iter().fold(0u32, |acc, bit| (acc << 1) | u32::from(bit));
        let mut widgetlines = Vec::new();
        widgetlines.push(Line::from(vec![Span::styled(format!("{:X} | {} | {}", decdata, bindata.clone(), decdata), Style::default().fg(Color::Yellow))]));
        widgetlines.push(Line::from(vec![Span::styled(bindata.replace('0', "○").replace('1', "●"), Style::default().fg(Color::Yellow))]));
    
        Paragraph::new(widgetlines)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn render_h_link(link: &Link, left: bool, area: Rect, buf: &mut Buffer) {
    let display_text = if left {
        format!("🭹{}", link.control)
    } else {
        format!("{}🭹", link.control)
    };

    // Create the styled text
    let body_text = Text::from(vec![Line::from(vec![
        Span::styled(display_text, Style::default().fg(Color::Yellow))
    ])]);

    // Render the paragraph
    Paragraph::new(body_text).render(area, buf);
}

fn render_v_link(link: &Link, area: Rect, buf: &mut Buffer) {
    // Create the styled text
    let body_text = Text::from(vec![
        Line::from(Span::styled(format!("{}", link.control), Style::default().fg(Color::Yellow))),
        Line::from(Span::styled(format!("|"), Style::default().fg(Color::Yellow)))
    ]);

    // Render the paragraph
    Paragraph::new(body_text).render(area, buf);
}