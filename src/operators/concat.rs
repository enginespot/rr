use std::{error::Error, sync::Arc};

use crate::{subscription::Subscription, Publisher, Subscriber};

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn concat(self, sources: Vec<Publisher<'a, T>>) -> Publisher<'a, T> {
        let func = move |subscriber: Box<Subscriber<'a, T>>| {
            let subscriber = Arc::new(subscriber);
            let len = sources.len();
            let mut subscriptions: Vec<Subscription> = Vec::new();
            for (index, source) in sources.into_iter().enumerate() {
                let on_next = {
                    let subscriber = subscriber.clone();
                    move |data: T| subscriber.on_next(data)
                };

                let on_error = {
                    let subscriber = subscriber.clone();
                    move |data: Box<dyn Error + Send>| subscriber.on_error(data)
                };

                let on_completed = {
                    let subscriber = subscriber.clone();
                    move |index: usize| {
                        move || {
                            if index == len - 1 {
                                return subscriber.on_completed();
                            } else {
                                return Ok(());
                            }
                        }
                    }
                };

                let subscription = source.subscribe(
                    Box::new(on_next),
                    Box::new(on_error),
                    Box::new(on_completed(index)),
                )?;
                subscriptions.push(subscription);
            }
            Ok(Subscription::create(Box::new(|| {
                for s in subscriptions {
                    s.unsubscribe()
                }
                Ok(())
            })))
        };

        Publisher::create(Box::new(func))
    }
}
