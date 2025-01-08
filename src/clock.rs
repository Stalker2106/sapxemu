use std::{
    sync::{mpsc::Sender, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

pub struct Clock {
    frequency: usize,                // Hertz
    state: bool,                     // Current clock state
    running: Arc<Mutex<bool>>,       // Shared running state
    thread_handle: Option<JoinHandle<()>>, // Handle to the clock thread
}

impl Clock {
    pub fn new(frequency: usize) -> Self {
        Self {
            frequency,
            state: false,
            running: Arc::new(Mutex::new(false)), // Initially paused
            thread_handle: None,
        }
    }

    pub fn start(&mut self, tx: Sender<()>) {
        let running = Arc::clone(&self.running);
        let frequency = self.frequency;

        // If a thread is already running, do nothing
        if self.thread_handle.is_some() {
            println!("Clock is already running.");
            return;
        }

        // Set the running flag to true
        *running.lock().unwrap() = true;

        // Spawn the clock thread
        self.thread_handle = Some(thread::spawn(move || {
            let interval = Duration::from_millis(1000 / frequency as u64);

            while *running.lock().unwrap() {
                println!("Clock pulse!");
                // Send a signal to the main thread
                if tx.send(()).is_err() {
                    println!("Main thread disconnected. Clock thread exiting.");
                    break;
                }
                thread::sleep(interval);
            }
        }));
    }

    pub fn pause(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            // Set the running flag to false
            *self.running.lock().unwrap() = false;

            // Wait for the thread to finish
            handle.join().unwrap();
            println!("Clock paused.");
        } else {
            println!("Clock is not running.");
        }
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }
}