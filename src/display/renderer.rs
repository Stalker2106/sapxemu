use std::{cell::RefCell, collections::HashMap, io::Stdout, rc::Rc};

use crossterm::{execute, terminal::{self, disable_raw_mode, LeaveAlternateScreen}};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, prelude::CrosstermBackend, Frame, Terminal};

use crate::{alu::ALU, bus::Bus, control::{control::ControlLine, controller::{self, Controller}}, link::Link, memory::{memory::RAM, register::{RORegister, RWRegister}}, pc::ProgramCounter};

const ICS_START_Y: u16 = 1;
const ICS_HEIGHT: u16 = 4;
const ICS_Y_SPACING: u16 = 2;
const ICS_WIDTH: u16 = 20;
const SIGNAL_WIDTH: u16 = 5;
const BUS_LINK_WIDTH: u16 = 10;
const BUS_WIDTH: u16 = 10;
const RAM_WIDTH: u16 = 20;
const RAM_HEIGHT: u16 = 40;

fn render_left(frame: &mut Frame, left_inner_layout: &Rc<[Rect]>, control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>, mar: &Rc<RefCell<RORegister>>, ir: &Rc<RefCell<RWRegister>>) {
    let mar_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(70),
        Constraint::Percentage(20),
    ])
    .split(left_inner_layout[2]);
    frame.render_widget(&*mar.borrow(), mar_layout[1]);
    let ir_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(70),
        Constraint::Percentage(20),
    ])
    .split(left_inner_layout[6]);
    frame.render_widget(&*ir.borrow(), ir_layout[1]);
}

fn render_right(frame: &mut Frame, right_inner_layout: &Rc<[Rect]>, control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>, pc: &ProgramCounter, reg_a: &Rc<RefCell<RWRegister>>, alu: &Rc<RefCell<ALU>>, reg_b: &Rc<RefCell<RWRegister>>, reg_out: &RORegister) {
    let pc_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(70),
        Constraint::Percentage(10),
    ])
    .split(right_inner_layout[0]);
    frame.render_widget(pc, pc_layout[1]);
    let reg_a_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(70),
        Constraint::Percentage(10),
    ])
    .split(right_inner_layout[2]);
    frame.render_widget(&*reg_a.borrow(), reg_a_layout[1]);
    let alu_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(70),
        Constraint::Percentage(10),
    ])
    .split(right_inner_layout[4]);
    frame.render_widget(&*alu.borrow(), alu_layout[1]);
    let reg_b_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(70),
        Constraint::Percentage(10),
    ])
    .split(right_inner_layout[6]);
    frame.render_widget(&*reg_b.borrow(), reg_b_layout[1]);
    let reg_out_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(70),
        Constraint::Percentage(10),
    ])
    .split(right_inner_layout[8]);
    frame.render_widget(reg_out, reg_out_layout[1]);
}

fn render(frame: &mut Frame, control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>, controller: &Controller, bus: &Rc<RefCell<Bus>>, pc: &ProgramCounter, alu: &Rc<RefCell<ALU>>, mar: &Rc<RefCell<RORegister>>, ram: &RAM, ir: &Rc<RefCell<RWRegister>>, reg_a: &Rc<RefCell<RWRegister>>, reg_b: &Rc<RefCell<RWRegister>>, reg_out: &RORegister) {
    let main_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(75),
        Constraint::Percentage(25),
    ])
    .split(frame.area());
    let computer_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(80),
        Constraint::Percentage(20),
    ])
    .split(main_layout[0]);
    let top_computer_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .split(computer_layout[0]);
    let left_inner_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(16),
        Constraint::Percentage(5),
        Constraint::Percentage(16),
        Constraint::Percentage(5),
        Constraint::Percentage(16),
        Constraint::Percentage(5),
        Constraint::Percentage(16),
        Constraint::Percentage(5),
        Constraint::Percentage(16),
    ])
    .split(top_computer_layout[0]);
    let right_inner_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(16),
        Constraint::Percentage(5),
        Constraint::Percentage(16),
        Constraint::Percentage(5),
        Constraint::Percentage(16),
        Constraint::Percentage(5),
        Constraint::Percentage(16),
        Constraint::Percentage(5),
        Constraint::Percentage(16),
    ])
    .split(top_computer_layout[2]);
    // Left
    render_left(frame, &left_inner_layout, control_links, mar, ir);
    // Bus
    frame.render_widget(&*bus.borrow(), top_computer_layout[1]);
    // Right
    render_right(frame, &right_inner_layout, control_links, pc, reg_a, alu, reg_b, reg_out);
    // RAM
    frame.render_widget( ram, main_layout[1]);
    // Controller
    let controller_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(30),
        Constraint::Percentage(70),
    ])
    .split(computer_layout[1]);
    frame.render_widget(controller, controller_layout[1]);
}

pub struct Renderer {
    terminal: Terminal<CrosstermBackend<Stdout>>
}

impl Renderer {
    pub fn new() -> Self {
        color_eyre::install().unwrap();
        let ren = Self {
            terminal: ratatui::init()
        };
        return ren;
    }

    pub fn draw(&mut self, control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>, controller: &Controller, bus: &Rc<RefCell<Bus>>, pc: &ProgramCounter, alu: &Rc<RefCell<ALU>>, mar: &Rc<RefCell<RORegister>>, ram: &RAM, ir: &Rc<RefCell<RWRegister>>, reg_a: &Rc<RefCell<RWRegister>>, reg_b: &Rc<RefCell<RWRegister>>, reg_out: &RORegister) {
        self.terminal.draw(|f| {
            // Pass required arguments to the render logic here
            render(f, control_links, controller, bus, pc, alu, mar, ram, ir, reg_a, reg_b, reg_out)
        }).unwrap();
    }

    pub fn stop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        self.terminal.show_cursor().unwrap();
    }

}