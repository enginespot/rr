use crate::{
    error::RRError, schedulers::Scheduler, subscription::Subscription, Publisher, Subscriber,
};
use crossbeam_channel::unbounded;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn subscribe_on<R>(self, scheduler: R) -> Publisher<'a, T>
    where
        R: Scheduler<'a> + 'a + Send,
    {
        let func = move |subscriber: Box<Subscriber<'a, T>>| {
            let (sender, receiver) = unbounded::<Subscription>();
            scheduler.schedule(Box::new(move || {
                let subscription = self.subscribe_subscriber(subscriber)?;
                sender
                    .send(subscription)
                    .map_err(|_e| RRError::SubscribeOnError)?;
                Ok(())
            }))?;

            Ok(Subscription::create(Box::new(move || {
                match receiver.recv() {
                    Ok(s) => {
                        s.unsubscribe();
                        Ok(())
                    },
                    Err(_) => Err(RRError::SubscriptionError),
                }
            })))
        };
        Publisher::create(Box::new(func))
    }
}
