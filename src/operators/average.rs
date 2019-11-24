use crate::{Publisher, Subscriber};
use num_traits::Num;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn average(self) -> Publisher<'a, T>
    where
        T: Num + PartialOrd + Copy,
    {
        let func = move |subscriber: Box<Subscriber<'a, T>>| {
            let subscriber = Arc::new(subscriber);
            let sum = T::zero();
            let count = T::one();
            let sum = Arc::new(Mutex::new(sum));
            let count = Arc::new(Mutex::new(count));

            let on_next = {
                let sum = sum.clone();
                let count = count.clone();
                move |data: T| {
                    *sum.lock().unwrap() = *sum.lock().unwrap() + data;
                    *count.lock().unwrap() = *count.lock().unwrap() + T::one();
                    Ok(())
                }
            };
            let on_error = {
                let subscriber = subscriber.clone();
                move |data: Box<dyn Error + Send>| subscriber.on_error(data)
            };
            let on_completed = {
                let subscriber = subscriber.clone();
                move || {
                    let sum = *sum.lock().unwrap();
                    let count = *count.lock().unwrap();
                    if count > T::zero() {
                        subscriber.on_next(sum / count)?;
                    }
                    subscriber.on_completed()
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
