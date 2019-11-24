use std::{error::Error, sync::Arc};

use crate::{Publisher, Subscriber};

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn do_on_next<FnNext>(self, next: Box<FnNext>) -> Publisher<'a, T>
    where
        FnNext: Fn() + 'a + Send + Sync,
    {
        let func = move |subscriber: Box<Subscriber<'a, T>>| {
            let subscriber = Arc::new(subscriber);
            let on_next = {
                let subscriber = subscriber.clone();
                move |data: T| {
                    next();
                    subscriber.on_next(data)
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
