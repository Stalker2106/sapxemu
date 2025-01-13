use std::{cell::RefCell, collections::HashMap, io::Stdout, rc::Rc};

use crossterm::{execute, terminal::{self, disable_raw_mode, LeaveAlternateScreen}};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, prelude::CrosstermBackend, Frame, Terminal};

use crate::{alu::ALU, bus::Bus, control::control::ControlLine, link::Link, memory::{memory::RAM, register::{RORegister, RWRegister}}, pc::ProgramCounter};

const ICS_START_Y: u16 = 1;
const ICS_HEIGHT: u16 = 4;
const ICS_Y_SPACING: u16 = 2;
const ICS_WIDTH: u16 = 20;
const SIGNAL_WIDTH: u16 = 5;
const BUS_LINK_WIDTH: u16 = 10;
const BUS_WIDTH: u16 = 10;
const RAM_WIDTH: u16 = 20;
const RAM_HEIGHT: u16 = 40;

fn render_left(frame: &mut Frame, left_inner_layout: Layout, control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>, mar: &Rc<RefCell<RORegister>>, ir: &Rc<RefCell<RWRegister>>) {
    let mararea = Rect::new(SIGNAL_WIDTH, ICS_START_Y+((ICS_HEIGHT+ICS_Y_SPACING)*1), ICS_WIDTH, ICS_HEIGHT);
    frame.render_widget(&*mar.borrow(), mararea);
    let ramarea = Rect::new(SIGNAL_WIDTH, ICS_START_Y+((ICS_HEIGHT+ICS_Y_SPACING)*2), ICS_WIDTH, ICS_HEIGHT);
    frame.render_widget(&*ir.borrow(), ramarea);
    let irarea = Rect::new(SIGNAL_WIDTH, ICS_START_Y+((ICS_HEIGHT+ICS_Y_SPACING)*3), ICS_WIDTH, ICS_HEIGHT);
    frame.render_widget(&*ir.borrow(), irarea);
}

fn render_right(frame: &mut Frame, right_inner_layout: Layout, control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>, pc: &ProgramCounter, reg_a: &Rc<RefCell<RWRegister>>, alu: &Rc<RefCell<ALU>>, reg_b: &Rc<RefCell<RWRegister>>, reg_out: &RORegister) {
    let pcarea = Rect::new(SIGNAL_WIDTH+ICS_WIDTH+BUS_LINK_WIDTH+BUS_WIDTH+BUS_LINK_WIDTH, ICS_START_Y, ICS_WIDTH, ICS_HEIGHT);
    frame.render_widget(pc, right_inner_layout[0]);
    let regaarea = Rect::new(SIGNAL_WIDTH+ICS_WIDTH+BUS_LINK_WIDTH+BUS_WIDTH+BUS_LINK_WIDTH, ICS_START_Y+((ICS_HEIGHT+ICS_Y_SPACING)*1), ICS_WIDTH, ICS_HEIGHT);
    frame.render_widget(&*reg_a.borrow(), regaarea);
    let aluarea = Rect::new(SIGNAL_WIDTH+ICS_WIDTH+BUS_LINK_WIDTH+BUS_WIDTH+BUS_LINK_WIDTH, ICS_START_Y+((ICS_HEIGHT+ICS_Y_SPACING)*2), ICS_WIDTH, ICS_HEIGHT);
    frame.render_widget(&*alu.borrow(), aluarea);
    let regbarea = Rect::new(SIGNAL_WIDTH+ICS_WIDTH+BUS_LINK_WIDTH+BUS_WIDTH+BUS_LINK_WIDTH, ICS_START_Y+((ICS_HEIGHT+ICS_Y_SPACING)*3), ICS_WIDTH, ICS_HEIGHT);
    frame.render_widget(&*reg_b.borrow(), regbarea);
    let regoutarea = Rect::new(SIGNAL_WIDTH+ICS_WIDTH+BUS_LINK_WIDTH+BUS_WIDTH+BUS_LINK_WIDTH, ICS_START_Y+((ICS_HEIGHT+ICS_Y_SPACING)*4), ICS_WIDTH, ICS_HEIGHT);
    frame.render_widget(reg_out, regoutarea);
}

fn render(frame: &mut Frame, control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>, bus: &Rc<RefCell<Bus>>, pc: &ProgramCounter, alu: &Rc<RefCell<ALU>>, mar: &Rc<RefCell<RORegister>>, ram: &RAM, ir: &Rc<RefCell<RWRegister>>, reg_a: &Rc<RefCell<RWRegister>>, reg_b: &Rc<RefCell<RWRegister>>, reg_out: &RORegister) {
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
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ])
    .split(top_computer_layout[0]);
    let right_inner_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ])
    .split(top_computer_layout[2]);
    // Left
    render_left(frame, left_inner_layout, control_links, mar, ir);
    // Bus
    let busarea = Rect::new(SIGNAL_WIDTH+ICS_WIDTH+BUS_LINK_WIDTH, 0, BUS_WIDTH, ICS_START_Y+((ICS_HEIGHT+ICS_Y_SPACING)*4)+ICS_START_Y);
    frame.render_widget(&*bus.borrow(), busarea);
    // Right
    render_right(frame, control_links, pc, reg_a, alu, reg_b, reg_out);
    // RAM
    let ramarea = Rect::new(SIGNAL_WIDTH+ICS_WIDTH+BUS_LINK_WIDTH+BUS_WIDTH+BUS_LINK_WIDTH+ICS_WIDTH+SIGNAL_WIDTH, 0, RAM_WIDTH, RAM_HEIGHT);
    frame.render_widget(right_inner_layout, ram, ramarea);
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

    pub fn draw(&mut self, control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>, bus: &Rc<RefCell<Bus>>, pc: &ProgramCounter, alu: &Rc<RefCell<ALU>>, mar: &Rc<RefCell<RORegister>>, ram: &RAM, ir: &Rc<RefCell<RWRegister>>, reg_a: &Rc<RefCell<RWRegister>>, reg_b: &Rc<RefCell<RWRegister>>, reg_out: &RORegister) {
        self.terminal.draw(|f| {
            // Pass required arguments to the render logic here
            render(f, control_links, bus, pc, alu, mar, ram, ir, reg_a, reg_b, reg_out)
        }).unwrap();
    }

    pub fn stop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        self.terminal.show_cursor().unwrap();
    }

}