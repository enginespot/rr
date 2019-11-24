use std::{error::Error, sync::Arc};

use crate::{Publisher, Subscriber};

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn find(
        self,
        predicate: Box<dyn Fn(&T) -> Result<bool, Box<dyn Error + Send>> + 'a + Send + Sync>,
    ) -> Publisher<'a, T> {
        let func = move |subscriber: Box<Subscriber<'a, T>>| {
            let subscriber = Arc::new(subscriber);

            let on_next = {
                let subscriber = subscriber.clone();

                move |data: T| {
                    match predicate(&data) {
                        Ok(true) =>
                            return subscriber
                                .on_next(data)
                                .and_then(|_| subscriber.on_completed()),
                        Ok(false) => return Ok(()),
                        Err(e) => return subscriber.on_error(e),
                    };
                }
            };

            let on_error = {
                let subscriber = subscriber.clone();

                move |e: Box<dyn Error + Send>| subscriber.on_error(e)
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
