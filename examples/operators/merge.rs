use rr::{Publisher, RRResult, Subscriber, Subscription};
use std::error::Error;

pub fn run() -> RRResult<Subscription> {
    let ob: Vec<i32> = vec![1, 2, 3, 4];

    let ob_1 = Publisher::from_iterable(ob);

    let on_next = |v: i32| {
        println!("{:?}", v);
        Ok(())
    };
    let on_error = |e: Box<dyn Error + Send>| {
        println!("{:?}", e);
        Ok(())
    };
    let on_completed = || {
        println!("merge done");
        Ok(())
    };

    let subscriber = Subscriber::create(
        Box::new(on_next),
        Box::new(on_error),
        Box::new(on_completed),
    );

    let ob_2: Vec<i32> = vec![5, 6, 7, 8];

    let ob_2 = Publisher::from_iterable(ob_2);

    let observer_merge = Publisher::merge(vec![ob_1, ob_2]);

    observer_merge
        //        .map(Box::new(|data: i32| data + 100))
        .subscribe_subscriber(Box::new(subscriber))
}
