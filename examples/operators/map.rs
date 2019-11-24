use std::error::Error;

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
        println!("map done");
        Ok(())
    };

    let subscriber = Subscriber::create(
        Box::new(on_next),
        Box::new(on_error),
        Box::new(on_completed),
    );

    let map = ob.map(Box::new(|i| Ok(i + 1)));
    map.subscribe_subscriber(Box::new(subscriber))
}
