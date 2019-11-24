use crate::schedulers::Scheduler;

use crate::{
    error::RRError,
    util::{RRResult, Task},
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use crossbeam_utils::{thread, thread::Scope};

pub struct NewThreadScheduler<'a> {
    sender: Sender<Box<Task<'a>>>,
    receiver: Receiver<Box<Task<'a>>>,
}

impl<'a> NewThreadScheduler<'a> {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded::<Box<Task<'a>>>();
        NewThreadScheduler { sender, receiver }
    }
}

impl<'a> Scheduler<'a> for NewThreadScheduler<'a> {
    fn schedule(&self, func: Box<Task<'a>>) -> RRResult<()> {
        self.sender.send(func).map_err(|_e| RRError::ScheduleError)
    }

    fn start(&self) -> RRResult<()> {
        let scope = thread::scope(|s: &Scope| {
            let handle = s.spawn(|_| {
                match self.receiver.recv() {
                    Ok(task) => task(),
                    Err(_) => Err(RRError::RecvError),
                }
            });
            handle.join()
        });
        match scope {
            Ok(_) => Ok(()),
            Err(_) => Err(RRError::ThreadPanic),
        }
    }

    fn stop(&self) -> RRResult<()> {
        println!("stop called");

        Ok(())
    }
}
