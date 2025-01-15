use std::{cell::RefCell, io::Stdout, rc::Rc};

use crossterm::{execute, terminal::{disable_raw_mode, LeaveAlternateScreen}};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, prelude::CrosstermBackend, Frame, Terminal};

use crate::{clock::Clock, computer::Computer, control::control::ControlLine};

use super::widgets::{render_all_links, render_bus_connection, render_h_link, render_ic_connection, render_ram_inspector, BusConnection, ICConnection};

fn render_left(frame: &mut Frame, left_inner_layout: &Rc<[Rect]>, clock: &Rc<RefCell<Clock>>, computer: &Computer) {
    let clock_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(70),
        Constraint::Percentage(20),
    ])
    .split(left_inner_layout[0]);
    frame.render_widget( &*clock.borrow(), clock_layout[1]);
    let mar_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(70),
        Constraint::Percentage(20),
    ])
    .split(left_inner_layout[2]);
    render_h_link(frame, &*computer.control_links[&ControlLine::MI].borrow(), true, mar_layout[0]);
    frame.render_widget(&*computer.mar.borrow(), mar_layout[1]);
    render_bus_connection(frame, BusConnection::Left, computer.control_links[&ControlLine::MI].borrow().get_state(),mar_layout[2]);
    let ram_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(70),
        Constraint::Percentage(20),
    ])
    .split(left_inner_layout[4]);
    let ram_links_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(ram_layout[0]);
    render_h_link(frame, &*computer.control_links[&ControlLine::RI].borrow(), true, ram_links_layout[0]);
    render_h_link(frame, &*computer.control_links[&ControlLine::RO].borrow(), true, ram_links_layout[1]);
    frame.render_widget(&computer.ram, ram_layout[1]);
    render_bus_connection(frame, BusConnection::Both, computer.control_links[&ControlLine::RO].borrow().get_state(), ram_layout[2]);
    let ir_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(70),
        Constraint::Percentage(20),
    ])
    .split(left_inner_layout[6]);
    let ir_links_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(ir_layout[0]);
    render_h_link(frame, &*computer.control_links[&ControlLine::II].borrow(), true, ir_links_layout[0]);
    render_h_link(frame, &*computer.control_links[&ControlLine::IO].borrow(), true, ir_links_layout[1]);
    frame.render_widget(&*computer.ir.borrow(), ir_layout[1]);
    render_bus_connection(frame, BusConnection::Both, computer.control_links[&ControlLine::II].borrow().get_state(), ir_layout[2]);
}

fn render_right(frame: &mut Frame, right_inner_layout: &Rc<[Rect]>, computer: &Computer) {
    let pc_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(70),
        Constraint::Percentage(10),
    ])
    .split(right_inner_layout[0]);
    let pc_links_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(33),
    ])
    .split(pc_layout[2]);
    render_h_link(frame, &*computer.control_links[&ControlLine::CO].borrow(), false, pc_links_layout[0]);
    render_h_link(frame, &*computer.control_links[&ControlLine::J].borrow(), false, pc_links_layout[1]);
    render_h_link(frame, &*computer.control_links[&ControlLine::CE].borrow(), false, pc_links_layout[2]);
    render_bus_connection(frame, BusConnection::Both, computer.control_links[&ControlLine::CO].borrow().get_state(),pc_layout[0]);
    frame.render_widget(&computer.pc, pc_layout[1]);
    let reg_a_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(70),
        Constraint::Percentage(10),
    ])
    .split(right_inner_layout[2]);
    let reg_a_links_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(reg_a_layout[2]);
    frame.render_widget(&*computer.reg_a.borrow(), reg_a_layout[1]);
    render_h_link(frame, &*computer.control_links[&ControlLine::AI].borrow(), false, reg_a_links_layout[0]);
    render_h_link(frame, &*computer.control_links[&ControlLine::AO].borrow(), false, reg_a_links_layout[1]);
    render_bus_connection(frame, BusConnection::Both, computer.control_links[&ControlLine::AI].borrow().get_state(),reg_a_layout[0]);
    render_ic_connection(frame, ICConnection::Down, false, right_inner_layout[3]);
    let alu_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(70),
        Constraint::Percentage(10),
    ])
    .split(right_inner_layout[4]);
    let alu_links_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(alu_layout[2]);
    frame.render_widget(&*computer.alu.borrow(), alu_layout[1]);
    render_h_link(frame, &*computer.control_links[&ControlLine::EO].borrow(), false, alu_links_layout[0]);
    render_h_link(frame, &*computer.control_links[&ControlLine::SU].borrow(), false, alu_links_layout[1]);
    render_bus_connection(frame, BusConnection::Both, computer.control_links[&ControlLine::EO].borrow().get_state(),alu_layout[0]);
    let reg_b_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(70),
        Constraint::Percentage(10),
    ])
    .split(right_inner_layout[6]);
    let reg_b_links_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(reg_b_layout[2]);
    render_ic_connection(frame, ICConnection::Up, false, right_inner_layout[5]);
    render_h_link(frame, &*computer.control_links[&ControlLine::BI].borrow(), false, reg_b_links_layout[0]);
    render_h_link(frame, &*computer.control_links[&ControlLine::BO].borrow(), false, reg_b_links_layout[1]);
    frame.render_widget(&*computer.reg_b.borrow(), reg_b_layout[1]);
    render_bus_connection(frame, BusConnection::Both, computer.control_links[&ControlLine::BI].borrow().get_state(),reg_b_layout[0]);
    let reg_out_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(70),
        Constraint::Percentage(10),
    ])
    .split(right_inner_layout[8]);
    frame.render_widget(&computer.reg_out, reg_out_layout[1]);
    render_h_link(frame, &*computer.control_links[&ControlLine::OI].borrow(), false, reg_out_layout[2]);
    render_bus_connection(frame, BusConnection::Right, computer.control_links[&ControlLine::OI].borrow().get_state(),reg_out_layout[0]);
}

fn render(frame: &mut Frame, clock: &Rc<RefCell<Clock>>, computer: &Computer) {
    let main_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(65),
        Constraint::Percentage(10),
        Constraint::Percentage(25),
    ])
    .split(frame.area());
    let inspector_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(75),
        Constraint::Percentage(25),
    ])
    .split(main_layout[3]);
    let computer_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(70),
        Constraint::Percentage(10),
        Constraint::Percentage(20),
    ])
    .split(main_layout[1]);
    let top_computer_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(43),
        Constraint::Percentage(15),
        Constraint::Percentage(42),
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
    render_left(frame, &left_inner_layout, clock, computer);
    // Bus
    frame.render_widget(&*computer.bus.borrow(), top_computer_layout[1]);
    // Right
    render_right(frame, &right_inner_layout, computer);
    // Inspector
    render_ram_inspector(frame, &computer.ram, inspector_layout[0]);
    frame.render_widget( &*clock.borrow(), inspector_layout[1]);
    // Controller
    let controller_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Length(2),
        Constraint::Percentage(70),
    ])
    .split(computer_layout[2]);
    render_all_links(frame, &computer.control_links, controller_layout[0]);
    frame.render_widget(&computer.controller, controller_layout[1]);
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

    pub fn draw(&mut self, clock: &Rc<RefCell<Clock>>, computer: &Computer) {
        self.terminal.draw(|f| {
            // Pass required arguments to the render logic here
            render(f, clock, computer)
        }).unwrap();
    }

    pub fn stop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        self.terminal.show_cursor().unwrap();
    }

}