extern crate time;

use std::collections::HashMap;
use util::Hasher;

const GC_INTERVAL: f64 = 10f64;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum PerfType {
    Update,
    Draw,
    VSync,
    Total,
}

pub struct Perf {
    interval_log: HashMap<PerfType, Vec<(f64,f64)>, Hasher>,
    previous_time: HashMap<PerfType, f64, Hasher>,
    previous_gc_time: f64,
}

impl Perf {
    pub fn new() -> Perf {
        Perf {
            interval_log: HashMap::<_,_,Hasher>::default(),
            previous_time: HashMap::<_,_,Hasher>::default(),
            previous_gc_time: 0f64,
        }
    }

    pub fn start_interval(&mut self, perf_type: PerfType) {
        self.previous_time.insert(perf_type, current_time());
    }

    pub fn end_interval(&mut self, perf_type: PerfType) {
        let time = current_time();
        let previous_time = self.previous_time.get(&perf_type).unwrap_or(&0f64).clone();
        if previous_time != 0f64 {
            let interval_entry = (time, time - previous_time);
            if self.interval_log.contains_key(&perf_type) {
                self.interval_log.get_mut(&perf_type).unwrap().push(interval_entry);
            } else {
                self.interval_log.insert(perf_type, vec![interval_entry]);
            }
        }
        self.previous_time.insert(perf_type, 0f64);
        self.check_gc(time);
    }

    pub fn prev_interval(&self, perf_type: PerfType) -> f64 {
        match self.interval_log.get(&perf_type) {
            Some(intervals) => {
                match (intervals.last()) {
                    Some(interval) => interval.1,
                    None => 0f64,
                }
            },
            None => 0f64,
        }
    }

    pub fn average_interval_sec(&self, perf_type: PerfType) -> f64 {
        self.average_interval(perf_type, 1f64)
    }

    pub fn average_interval(&self, perf_type: PerfType, time: f64) -> f64 {
        match self.interval_log.get(&perf_type) {
            Some(intervals) => {
                let mut n_intervals = 0;
                let mut total_interval = 0f64;
                let deadline_time = current_time() - time;
                for (log_time, interval) in intervals.iter().rev() {
                    if log_time <= &deadline_time {
                        break;
                    }
                    n_intervals += 1;
                    total_interval += interval;
                }
                total_interval / n_intervals as f64
            },
            None => 0f64,
        }
    }

    fn check_gc(&mut self, current_time: f64) {
        if self.previous_gc_time + GC_INTERVAL <= current_time {
            for (_, interval_entries) in self.interval_log.iter_mut() {
                interval_entries.retain(|&interval_entry| interval_entry.0 + GC_INTERVAL > current_time);
            }
            self.previous_gc_time = current_time;
        }
    }
}

fn current_time() -> f64 {
    let nano_time = time::precise_time_ns();
    nano_time as f64 / 1000_000_000f64
}
