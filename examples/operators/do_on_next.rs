use std::{error::Error, thread};

use rr::{Publisher, RRResult, Subscriber, Subscription};

pub fn run() -> RRResult<Subscription> {
    let ob: Vec<i32> = vec![1, 2, 3, 4];

    let ob = Publisher::from_iterable(ob);

    let on_next = |v: i32| {
        println!("{:?}", v);
        Ok(())
    };
    let on_error = |e: Box<dyn Error + Send>| {
        println!("{:?}", e);
        Ok(())
    };
    let on_completed = || {
        println!("do on next done");
        Ok(())
    };

    let subscriber = Subscriber::create(
        Box::new(on_next),
        Box::new(on_error),
        Box::new(on_completed),
    );
    ob.do_on_next(Box::new(|| {
        println!(
            "processing item on thread:{:?}",
            thread::current().name().unwrap()
        );
    }))
    .map(Box::new(|i| Ok(i + 1)))
    .subscribe_subscriber(Box::new(subscriber))
}
