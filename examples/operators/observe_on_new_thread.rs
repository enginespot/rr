use std::{error::Error, thread};

use rr::{Publisher, RRResult, Schedulers, Subscriber, Subscription};

pub fn run() -> RRResult<Subscription> {
    let ob: Vec<i32> = vec![1, 2, 3, 4];

    let ob = Publisher::from_iterable(ob);

    let on_next = |_| {
        println!("observe_on next:{:?}", thread::current().name());
        Ok(())
    };
    let on_error = |e: Box<dyn Error + Send>| {
        println!("{:?}", e);
        Ok(())
    };
    let on_completed = || {
        println!("observe_on done:{:?}", thread::current().name());
        Ok(())
    };

    let subscriber = Subscriber::create(
        Box::new(on_next),
        Box::new(on_error),
        Box::new(on_completed),
    );
    ob.publish_on(Schedulers::new_thread())
        .subscribe_on(Schedulers::new_thread())
        .subscribe_subscriber(Box::new(subscriber))
}
