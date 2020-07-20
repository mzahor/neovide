use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use log::trace;

lazy_static! {
    pub static ref REDRAW_SCHEDULER: RedrawScheduler = RedrawScheduler::new();
}

pub struct RedrawScheduler {
    frame_queued: AtomicBool,
    scheduled_frame: Mutex<Option<Instant>>,
}

impl RedrawScheduler {
    pub fn new() -> RedrawScheduler {
        RedrawScheduler {
            frame_queued: AtomicBool::new(true),
            scheduled_frame: Mutex::new(None),
        }
    }

    pub fn schedule(&self, new_scheduled: Instant) {
        trace!("Redraw scheduled for {:?}", new_scheduled);
        let mut scheduled_frame = self.scheduled_frame.lock().unwrap();

        if let Some(previous_scheduled) = *scheduled_frame {
            if new_scheduled < previous_scheduled {
                *scheduled_frame = Some(new_scheduled);
            }
        } else {
            *scheduled_frame = Some(new_scheduled);
        }
    }

    pub fn queue_next_frame(&self) {
        trace!("Next frame queued");
        self.frame_queued
            .store(true, Ordering::Relaxed);
    }

    pub fn should_draw(&self) -> bool {
        let frame_queued = self.frame_queued.load(Ordering::Relaxed);

        if frame_queued {
            self.frame_queued
                .store(false, Ordering::Relaxed);
            true
        } else {
            let mut next_scheduled_frame = self.scheduled_frame.lock().unwrap();

            if let Some(scheduled_frame) = *next_scheduled_frame {
                if scheduled_frame < Instant::now() {
                    *next_scheduled_frame = None;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}
