use crate::{subscription::Subscription, Publisher, Subscriber};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

impl<'a, T: 'a + Send> Publisher<'a, Publisher<'a, T>> {
    pub fn flatten(self) -> Publisher<'a, T> {
        let func = move |subscriber: Box<Subscriber<'a, T>>| {
            let subscriber = Arc::new(subscriber);
            let noop = || Ok(());

            let subscriptions: Vec<Subscription> = Vec::new();
            let subscriptions = Arc::new(Mutex::new(subscriptions));

            let on_error = {
                let subscriber = subscriber.clone();
                move |data: Box<dyn Error + Send>| subscriber.on_error(data)
            };

            let on_completed = {
                let subscriber = subscriber.clone();
                move || subscriber.on_completed()
            };

            let on_next = {
                let subscriptions = subscriptions.clone();
                move |publisher: Publisher<'a, T>| {
                    let inner_on_next = {
                        let subscriber = subscriber.clone();
                        move |d: T| subscriber.on_next(d)
                    };
                    let inner_on_error = {
                        let subscriber = subscriber.clone();

                        move |e: Box<dyn Error + Send>| subscriber.on_error(e)
                    };
                    let mut subscriptions = subscriptions.lock().unwrap();
                    subscriptions.push(publisher.subscribe(
                        Box::new(inner_on_next),
                        Box::new(inner_on_error),
                        Box::new(noop),
                    )?);
                    Ok(())
                }
            };

            {
                let subscriptions = subscriptions.clone();
                let mut subscriptions = subscriptions.lock().unwrap();

                subscriptions.push(self.subscribe(
                    Box::new(on_next),
                    Box::new(on_error),
                    Box::new(on_completed),
                )?);
            }

            let subscriptions = subscriptions.clone();

            Ok(Subscription::create(Box::new(move || {
                let subscriptions = subscriptions.lock().unwrap();
                for s in subscriptions.iter() {
                    s.unsubscribe();
                }
                Ok(())
            })))
        };

        Publisher::create(Box::new(func))
    }
}
