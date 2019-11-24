use std::{error::Error, sync::Arc};

use crate::{schedulers::Scheduler, Publisher, Subscriber};

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn publish_on<R>(self, scheduler: R) -> Publisher<'a, T>
    where
        R: Scheduler<'a> + 'a + Send + Sync,
    {
        let func = move |subscriber: Box<Subscriber<'a, T>>| {
            let subscriber = Arc::new(subscriber);
            let scheduler = Arc::new(scheduler);
            scheduler.start()?;
            let on_error = {
                let scheduler = scheduler.clone();
                let subscriber = subscriber.clone();
                move |data: Box<dyn Error + Send>| {
                    let subscriber = subscriber.clone();
                    scheduler.schedule(Box::new(move || subscriber.on_error(data)))
                }
            };
            let on_completed = {
                let scheduler = scheduler.clone();
                let subscriber = subscriber.clone();
                move || {
                    let subscriber = subscriber.clone();
                    scheduler.schedule(Box::new(move || subscriber.on_completed()))
                }
            };
            let on_next = {
                let scheduler = scheduler.clone();
                let subscriber = subscriber.clone();
                move |data: T| {
                    let subscriber = subscriber.clone();
                    scheduler.schedule(Box::new(move || subscriber.on_next(data)))
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
