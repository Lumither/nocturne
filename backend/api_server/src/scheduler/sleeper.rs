use chrono::{DateTime, Utc};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct SkippableSleeper {
    pub duration: Duration,
    state: Arc<(Mutex<bool>, Condvar)>,
}

impl SkippableSleeper {
    pub fn new(duration: Duration) -> Self {
        SkippableSleeper {
            duration,
            state: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    // Blocks until the duration has passed or skip() is called
    pub fn start(&self) {
        let (lock, cvar) = &*self.state;
        let mut received = lock.lock().unwrap();
        let state_clone = Arc::clone(&self.state);
        let duration_clone = self.duration;
        thread::spawn(move || {
            thread::sleep(duration_clone);
            let (lock, cvar) = &*state_clone;
            let mut received = lock.lock().unwrap();
            *received = true;
            cvar.notify_one();
        });
        while !*received {
            received = cvar.wait(received).unwrap();
        }
    }

    // Skip the blocking delay
    pub fn skip(&self) {
        let (lock, cvar) = &*self.state;
        let mut received = lock.lock().unwrap();
        *received = true;
        cvar.notify_one();
    }
}

impl From<DateTime<Utc>> for SkippableSleeper {
    fn from(value: DateTime<Utc>) -> Self {
        let from_utc_timestamp = value.timestamp() as u64;
        let curr_utc_timestamp = Utc::now().timestamp() as u64;
        SkippableSleeper::new(Duration::from_secs(from_utc_timestamp - curr_utc_timestamp))
    }
}

#[cfg(test)]
mod test {
    use crate::scheduler::sleeper::SkippableSleeper;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn usage() {
        let sleeper = SkippableSleeper::new(Duration::new(10, 0));
        thread::spawn({
            let sleeper = sleeper.clone();
            move || {
                println!("Sleeping...");
                sleeper.start();
                println!("Woke up!");
            }
        });
        println!("Start Sleeping...");
        thread::sleep(Duration::from_secs(4));
        println!("Skipping sleep...");
        sleeper.skip();
    }
}
