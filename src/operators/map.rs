use std::{error::Error, sync::Arc};

use crate::{Publisher, Subscriber};

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn map<R>(
        self,
        cb: Box<dyn Fn(T) -> Result<R, Box<dyn Error + Send>> + 'a + Send + Sync>,
    ) -> Publisher<'a, R>
    where
        R: 'a + Send,
    {
        let func = move |subscriber: Box<Subscriber<'a, R>>| {
            let subscriber = Arc::new(subscriber);
            let on_next = {
                let subscriber = subscriber.clone();
                move |data: T| {
                    match cb(data) {
                        Ok(data) => return subscriber.on_next(data),
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
