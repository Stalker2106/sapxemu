use std::{
    sync::{mpsc::Sender, Arc, Mutex}, thread::{self, JoinHandle}, time::Duration
};

pub struct Clock {
    frequency: usize,                // Hertz
    running: Arc<Mutex<bool>>,       // Shared running state
    thread_handle: Option<JoinHandle<()>>, // Handle to the clock thread
}

impl Clock {
    pub fn new(frequency: usize) -> Self {
        Self {
            frequency,
            running: Arc::new(Mutex::new(false)), // Initially paused
            thread_handle: None,
        }
    }

    pub fn start(&mut self, tx: Sender<bool>) {
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
            let half_interval = Duration::from_millis((1000 / frequency as u64) / 2);

            while *running.lock().unwrap() {
                // Send a signal to the main thread
                if tx.send(true).is_err() {
                    println!("Main thread disconnected. Clock thread exiting.");
                    break;
                }
                thread::sleep(half_interval);
                if tx.send(false).is_err() {
                    println!("Main thread disconnected. Clock thread exiting.");
                    break;
                }
                thread::sleep(half_interval);
            }
        }));
    }

    pub fn stop(&mut self) {
        // Set the running flag to false to stop the clock thread
        if let Some(handle) = self.thread_handle.take() {
            *self.running.lock().unwrap() = false;

            // Wait for the clock thread to finish
            handle.join().unwrap();
        } else {
            println!("Clock is not running.");
        }
    }
}

pub trait ClockDriven {
    fn on_clock_high(&mut self);
}