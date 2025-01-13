use std::{cell::RefCell, collections::HashMap, io::Error, process::exit, rc::Rc};

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{control::control::ControlLine, link::Link};


pub fn handle_keyboard(all_control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>) -> Result<(), Error> {
    if event::poll(std::time::Duration::from_secs(1))? {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Esc => {
                    all_control_links[&ControlLine::HLT].borrow_mut().set_endpoint("CONTROLLER".to_string(), true);
                }
                _ => {
                    println!("Key pressed: {:?}", code);
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}