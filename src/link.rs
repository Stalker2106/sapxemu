use std::collections::HashMap;

pub struct Link {
    endpoints: HashMap<String, bool>,
    callbacks: Vec<Box<dyn Fn()>>
}

impl Link {
    pub fn new() -> Self {
        Self {
            endpoints: HashMap::new(),
            callbacks: Vec::new()
        }
    }

    pub fn add_endpoint(&mut self, ep_indentifier: String) {
        self.endpoints.insert(ep_indentifier, false);
    }

    pub fn set_endpoint(&mut self, ep_indentifier: String, state: bool) {
        if let Some(value) = self.endpoints.get_mut(&ep_indentifier) {
            *value = state;
        }
        // Run callbacks if any
        for cb in &self.callbacks {
            cb();
        }
    }

    pub fn add_callback(&mut self, callback: Box<dyn Fn()>) {
        self.callbacks.push(callback);
    }

    pub fn get_state(&self) -> bool {
        for (_ep_id, val) in &self.endpoints {
            if *val {
                return true;
            }
        }
        return false;
    }
}