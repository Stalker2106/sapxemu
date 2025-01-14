use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{clock::ClockDriven, control::{control::ControlLine, sequencer::Sequencer}, display::renderer, link::Link};

// Controller
pub struct Controller<'a> {
    sequencer: Rc<RefCell<Sequencer>>,
    // Links
    pub control_links: &'a HashMap<ControlLine, Rc<RefCell<Link>>>
}

impl<'a> Controller<'a> {
    pub fn new(
        sequencer: Rc<RefCell<Sequencer>>,
        all_control_links: &'a HashMap<ControlLine, Rc<RefCell<Link>>>
    ) -> Self {
        for (_line, link) in all_control_links {
            link.borrow_mut().add_endpoint("CONTROLLER".to_string());
        }
        Self {
            sequencer,
            control_links: all_control_links
        }
    }

    pub fn drive_step_controls(&mut self, microcode_step: Vec<ControlLine>, state: bool) {
        for control in microcode_step {
            self.control_links[&control].borrow_mut().set_endpoint("CONTROLLER".to_string(), state);
        }
    }
}

impl<'a> ClockDriven for Controller<'a> {
    fn on_clock_pulse(&mut self) {
        // Drive Prev step controls low
        let prev_step_controls = self.sequencer.borrow().get_prev_step_controls();
        self.drive_step_controls(prev_step_controls, false);
        // Run current step
        let step_controls = self.sequencer.borrow().get_current_step_controls();
        self.drive_step_controls(step_controls, true);
    }
}