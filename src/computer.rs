use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::alu::ALU;
use crate::bitvecutils::convert_ramdump_to_bitvec;
use crate::bus::{Bus, BusSelector};
use crate::clock::ClockDriven;
use crate::config::{OPCODE_SIZE, WORD_SIZE};
use crate::control::control::ControlLine;
use crate::control::controller::Controller;
use crate::link::Link;
use crate::memory::memory::RAM;
use crate::pc::ProgramCounter;
use crate::memory::register::{RORegister, RWRegister};
use crate::control::sequencer::{self, Sequencer};

fn filter_control_lines(
    map: &HashMap<ControlLine, Rc<RefCell<Link>>>,
    keys: &[ControlLine],
) -> HashMap<ControlLine, Rc<RefCell<Link>>> {
    keys.iter()
        .filter_map(|key| map.get(key).map(|value| (key.clone(), Rc::clone(value))))
        .collect()
}

fn deep_clone_hashmap(
    map: &HashMap<ControlLine, Rc<RefCell<Link>>>,
) -> HashMap<ControlLine, Rc<RefCell<Link>>> {
    map.iter()
        .map(|(key, value)| (key.clone(), Rc::clone(value)))
        .collect()
}

pub struct Computer {
    pub control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
    pub bus: Rc<RefCell<Bus>>,
    pub pc: ProgramCounter,
    pub reg_a: Rc<RefCell<RWRegister>>,
    pub reg_b: Rc<RefCell<RWRegister>>,
    pub alu: Rc<RefCell<ALU>>,
    pub mar: Rc<RefCell<RORegister>>,
    pub ram: RAM,
    pub ir: Rc<RefCell<RWRegister>>,
    pub reg_out: RORegister,
    pub sequencer: Rc<RefCell<Sequencer>>,
    pub controller: Controller,
}

impl Computer {
    pub fn new(ramdump: Vec<u8>) -> Self {
        let control_links = Self::init_control_links();
        let bus = Rc::new(RefCell::new(Bus::new()));
        let sequencer = Rc::new(RefCell::new(Sequencer::new()));
        let reg_a = Self::init_register_a(filter_control_lines(&control_links, &[ControlLine::AI, ControlLine::AO]), Rc::clone(&bus));
        let reg_b = Self::init_register_b(filter_control_lines(&control_links, &[ControlLine::BI, ControlLine::BO]), Rc::clone(&bus)); // Fixed typo here
        let mar = Self::init_memory_address_register(filter_control_lines(&control_links, &[ControlLine::MI]), Rc::clone(&bus));

        Self {
            bus: Rc::clone(&bus),
            pc: Self::init_program_counter(filter_control_lines(&control_links, &[ControlLine::CO, ControlLine::J, ControlLine::CE]), Rc::clone(&bus)),
            alu: Self::init_alu(filter_control_lines(&control_links, &[ControlLine::EO, ControlLine::SU]), Rc::clone(&bus), Rc::clone(&reg_a), Rc::clone(&reg_b)),
            reg_a,
            reg_b,
            ram: Self::init_ram(filter_control_lines(&control_links, &[ControlLine::RI, ControlLine::RO]), Rc::clone(&bus), Rc::clone(&mar), ramdump),
            mar,
            ir: Self::init_instruction_register(filter_control_lines(&control_links, &[ControlLine::II, ControlLine::IO]), Rc::clone(&bus)),
            reg_out: Self::init_output_register(filter_control_lines(&control_links, &[ControlLine::OI]), Rc::clone(&bus)),
            controller: Self::init_controller(deep_clone_hashmap(&control_links), Rc::clone(&sequencer)),
            sequencer,
            control_links,
        }
    }

    fn init_control_links() -> HashMap<ControlLine, Rc<RefCell<Link>>> {
        let mut control_links = HashMap::new();
        for line in [
            ControlLine::CLK, ControlLine::HLT, ControlLine::MI, ControlLine::RI, ControlLine::RO,
            ControlLine::II, ControlLine::IO, ControlLine::AI, ControlLine::AO, ControlLine::EO,
            ControlLine::SU, ControlLine::BI, ControlLine::BO, ControlLine::OI, ControlLine::CE,
            ControlLine::CO, ControlLine::J, ControlLine::FI,
        ] {
            control_links.insert(line.clone(), Rc::new(RefCell::new(Link::new(line))));
        }
        control_links
    }

    fn init_program_counter(
        control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
        bus: Rc<RefCell<Bus>>,
    ) -> ProgramCounter {
        ProgramCounter::new(control_links, bus)
    }

    fn init_register_a(
        control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
        bus: Rc<RefCell<Bus>>,
    ) -> Rc<RefCell<RWRegister>> {
        Rc::new(RefCell::new(RWRegister::new(
            "A Register".to_string(),
            WORD_SIZE,
            bus,
            BusSelector::LSB,
            ControlLine::AI,
            ControlLine::AO,
            control_links,
        )))
    }

    fn init_register_b(
        control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
        bus: Rc<RefCell<Bus>>,
    ) -> Rc<RefCell<RWRegister>> {
        Rc::new(RefCell::new(RWRegister::new(
            "B Register".to_string(),
            WORD_SIZE,
            bus,
            BusSelector::LSB,
            ControlLine::BI,
            ControlLine::BO,
            control_links,
        )))
    }

    fn init_alu(
        control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
        bus: Rc<RefCell<Bus>>,
        reg_a: Rc<RefCell<RWRegister>>,
        reg_b: Rc<RefCell<RWRegister>>,
    ) -> Rc<RefCell<ALU>> {
        Rc::new(RefCell::new(ALU::new(control_links, bus, reg_a, reg_b)))
    }

    fn init_memory_address_register(
        control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
        bus: Rc<RefCell<Bus>>,
    ) -> Rc<RefCell<RORegister>> {
        Rc::new(RefCell::new(RORegister::new(
            "Mem. Address Reg.".to_string(),
            WORD_SIZE - OPCODE_SIZE,
            bus,
            BusSelector::LSB,
            ControlLine::MI,
            control_links,
        )))
    }

    fn init_ram(
        control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
        bus: Rc<RefCell<Bus>>,
        mar: Rc<RefCell<RORegister>>,
        ramdump: Vec<u8>,
    ) -> RAM {
        RAM::new(control_links, bus, mar, &convert_ramdump_to_bitvec(&ramdump))
    }

    fn init_instruction_register(
        control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
        bus: Rc<RefCell<Bus>>,
    ) -> Rc<RefCell<RWRegister>> {
        Rc::new(RefCell::new(RWRegister::new(
            "Instruction Reg.".to_string(),
            OPCODE_SIZE,
            bus,
            BusSelector::HSB,
            ControlLine::II,
            ControlLine::IO,
            control_links,
        )))
    }

    fn init_output_register(
        control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
        bus: Rc<RefCell<Bus>>,
    ) -> RORegister {
        RORegister::new(
            "Output Register".to_string(),
            WORD_SIZE,
            bus,
            BusSelector::LSB,
            ControlLine::OI,
            control_links,
        )
    }

    fn init_controller(
        control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
        sequencer: Rc<RefCell<Sequencer>>,
    ) -> Controller {
        Controller::new(control_links, sequencer)
    }

    pub fn on_clock_high(&mut self, clock_state: bool) {
        if clock_state {
            self.controller.on_clock_high();
            self.pc.on_clock_high();
            self.mar.borrow_mut().on_clock_high();
            self.ram.on_clock_high();
            self.ir.borrow_mut().on_clock_high();
            self.reg_a.borrow_mut().on_clock_high();
            self.reg_b.borrow_mut().on_clock_high();
            self.reg_out.on_clock_high();
        } else {
            self.controller.on_clock_low();
            self.sequencer.borrow_mut().increment_step(&self.ir);
        }
    }
}
