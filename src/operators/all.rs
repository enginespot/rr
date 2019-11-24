use std::{error::Error, sync::Arc};

use crate::{Publisher, Subscriber};

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn all<F>(self, predicate: Box<F>) -> Publisher<'a, bool>
    where
        F: for<'c> Fn(&'c T) -> bool + 'a + Send + Sync,
    {
        let func = move |subscriber: Box<Subscriber<'a, bool>>| {
            let subscriber = Arc::new(subscriber);

            let on_next = {
                let subscriber = subscriber.clone();
                move |data: T| {
                    if !predicate(&data) {
                        subscriber
                            .on_next(false)
                            .and_then(|_| subscriber.on_completed())
                    } else {
                        Ok(())
                    }
                }
            };
            let on_error = {
                let subscriber = subscriber.clone();
                move |data: Box<dyn Error + Send>| subscriber.on_error(data)
            };
            let on_completed = {
                let subscriber = subscriber.clone();
                move || {
                    subscriber
                        .on_next(true)
                        .and_then(|_| subscriber.on_completed())
                }
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
