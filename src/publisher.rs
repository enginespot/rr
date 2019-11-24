use crate::{
    subscription::Subscription,
    util::{FnComplete, FnError, FnNext, RRResult},
    Subscriber,
};

pub struct Publisher<'a, T: 'a + Send> {
    subscriber: Box<dyn FnOnce(Box<Subscriber<'a, T>>) -> RRResult<Subscription> + 'a + Send>,
}

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn create(
        subscriber: Box<dyn FnOnce(Box<Subscriber<'a, T>>) -> RRResult<Subscription> + 'a + Send>,
    ) -> Publisher<'a, T> {
        Publisher { subscriber }
    }

    pub fn from_iterable(list: Vec<T>) -> Publisher<'a, T> {
        let func = move |o: Box<Subscriber<'a, T>>| {
            let iter = list.into_iter();
            let subscription = Subscription::new();
            for data in iter {
                if subscription.is_unsubscribed() {
                    break;
                } else {
                    o.on_next(data)?
                }
            }
            if !subscription.is_unsubscribed() {
                o.on_completed()?;
            }
            Ok(subscription)
        };

        Publisher::create(Box::new(func))
    }

    pub fn cancel(&self) {}

    pub fn subscribe(
        self,
        on_next: Box<FnNext<'a, T>>,
        on_error: Box<FnError<'a>>,
        on_completed: Box<FnComplete<'a>>,
    ) -> RRResult<Subscription> {
        (self.subscriber)(Box::new(Subscriber::create(
            on_next,
            on_error,
            on_completed,
        )))
    }

    pub fn subscribe_subscriber(
        self,
        subscriber: Box<Subscriber<'a, T>>,
    ) -> RRResult<Subscription> {
        (self.subscriber)(subscriber)
    }
}
