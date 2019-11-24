use std::error::Error;

use rr::{Publisher, RRResult, Subscriber, Subscription};

pub fn run() -> RRResult<Subscription> {
    let ob: Vec<i32> = vec![1, 2, 3, 4];

    let ob = Publisher::from_iterable(ob);

    let on_next = |v: &str| {
        println!("{:?}", v);
        Ok(())
    };
    let on_error = |e: Box<dyn Error + Send>| {
        println!("{:?}", e);
        Ok(())
    };
    let on_completed = || {
        println!("flatten done");

        Ok(())
    };

    let subscriber = Subscriber::create(
        Box::new(on_next),
        Box::new(on_error),
        Box::new(on_completed),
    );

    let publisher = ob
        .map(Box::new(|_| {
            let data = vec!["a", "b", "c", "d"];
            Ok(Publisher::from_iterable(data))
        }))
        .flatten();

    publisher.subscribe_subscriber(Box::new(subscriber))
}
