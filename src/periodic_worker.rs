use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

pub struct PeriodicWorker {
    thread: Option<thread::JoinHandle<()>>,
    active: Arc<(Mutex<bool>, Condvar)>,
}

impl PeriodicWorker {
    // Panic if the `interval.is_zero()` is `true`.
    #[allow(clippy::mutex_atomic)]
    pub fn new(callback: impl Fn() -> bool + Send + Sync + 'static, interval: Duration) -> Self {
        if interval.is_zero() {
            panic!("PeriodicWorker: the interval cannot be zero")
        }

        let active = Arc::new((Mutex::new(true), Condvar::new()));

        Self {
            active: active.clone(),
            thread: Some(thread::spawn(move || loop {
                let guard = active.0.lock().unwrap();
                let (_, res) = active
                    .1
                    .wait_timeout_while(guard, interval, |active| *active)
                    .unwrap();

                if !res.timed_out() || !callback() {
                    return;
                }
            })),
        }
    }
}

impl Drop for PeriodicWorker {
    #[allow(clippy::mutex_atomic)]
    fn drop(&mut self) {
        *self.active.0.lock().unwrap() = false;
        self.active.1.notify_all();
        self.thread
            .take()
            .unwrap()
            .join()
            .expect("PeriodicWorker: worker thread panicked");
    }
}
