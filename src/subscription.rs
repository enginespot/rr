use crate::util::RRResult;

pub struct Subscription {
    action: Box<dyn FnOnce() -> RRResult<()> + Send + Sync>,
    unsubscribed: bool,
}

impl Subscription {
    pub fn is_unsubscribed(&self) -> bool {
        self.unsubscribed
    }

    pub fn unsubscribe(&self) {}

    pub fn new() -> Subscription {
        Subscription {
            action: Box::new(|| Ok(())),
            unsubscribed: true,
        }
    }

    pub fn create(action: Box<dyn FnOnce() -> RRResult<()> + Send + Sync>) -> Subscription {
        Subscription {
            action,
            unsubscribed: false,
        }
    }

    pub fn n() -> Subscription {
        Subscription {
            action: Box::new(|| Ok(())),
            unsubscribed: true,
        }
    }
}
