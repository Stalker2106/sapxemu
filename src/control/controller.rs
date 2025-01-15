use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{clock::ClockDriven, control::{control::ControlLine, sequencer::Sequencer}, display::renderer, link::Link};

// Controller
pub struct Controller {
    sequencer: Rc<RefCell<Sequencer>>,
    // Links
    pub control_links: HashMap<ControlLine, Rc<RefCell<Link>>>
}

impl Controller {
    pub fn new(
        control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
        sequencer: Rc<RefCell<Sequencer>>
    ) -> Self {
        for (_line, link) in &control_links {
            link.borrow_mut().add_endpoint("CONTROLLER".to_string());
        }
        Self {
            sequencer,
            control_links
        }
    }

    pub fn drive_step_controls(&mut self, microcode_step: Vec<ControlLine>, state: bool) {
        for control in microcode_step {
            self.control_links[&control].borrow_mut().set_endpoint("CONTROLLER".to_string(), state);
        }
    }

    pub fn on_clock_low(&mut self) {
        // drive all current signals low
        let step_controls = self.sequencer.borrow().get_current_step_controls();
        self.drive_step_controls(step_controls, false);
    }
}

impl ClockDriven for Controller {
    fn on_clock_high(&mut self) {
        // Run current step
        let step_controls = self.sequencer.borrow().get_current_step_controls();
        self.drive_step_controls(step_controls, true);
    }
}