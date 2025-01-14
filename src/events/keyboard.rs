use std::{cell::RefCell, collections::HashMap, io::Error, process::exit, rc::Rc};

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{control::control::ControlLine, display::renderer::Renderer, link::Link};


pub fn handle_keyboard(renderer: &Rc<RefCell<Renderer>>, all_control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>) -> Result<(), Error> {
    if event::poll(std::time::Duration::from_secs(1))? {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Esc => {
                    renderer.borrow_mut().stop();
                    std::process::exit(1);
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