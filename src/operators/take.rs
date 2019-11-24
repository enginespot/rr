use crate::{subscription::Subscription, Publisher, Subscriber};
use crossbeam_utils::atomic::AtomicCell;
use std::{error::Error, sync::Arc};

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn take(self, n: usize) -> Publisher<'a, T> {
        let func = move |subscriber: Box<Subscriber<'a, T>>| {
            let subscriber = Arc::new(subscriber);
            {
                let subscriber = subscriber.clone();
                if n == 0 {
                    return subscriber.on_completed().map(|_| {
                        return Subscription::n();
                    });
                }
            }
            let i = AtomicCell::new(1usize);
            let on_next = {
                let subscriber = subscriber.clone();
                move |data: T| {
                    subscriber.on_next(data).and_then(|_| {
                        i.fetch_add(1);
                        if i.load() > n {
                            return subscriber.on_completed();
                        } else {
                            return Ok(());
                        }
                    })
                }
            };
            let on_error = {
                let subscriber = subscriber.clone();
                move |data: Box<dyn Error + Send>| subscriber.on_error(data)
            };

            let on_completed = {
                let subscriber = subscriber.clone();
                move || subscriber.on_completed()
            };

            self.subscribe(
                Box::new(on_next),
                Box::new(on_error),
                Box::new(on_completed),
            )
        };

        Publisher::create(Box::new(func))
    }
}
