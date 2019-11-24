use std::error::Error;

use rr::{Publisher, RRResult, Subscription};

pub fn run() -> RRResult<Subscription> {
    let map_1: Vec<i32> = vec![1, 2, 3, 4];

    let observer_1 = Publisher::from_iterable(map_1);

    let on_next = |v: bool| {
        println!("all next {:?}", v);

        Ok(())
    };
    let on_error = |e: Box<dyn Error + Send>| {
        println!("{:?}", e);
        Ok(())
    };
    let on_completed = || {
        println!("all done");
        Ok(())
    };

    //    let on_subscribe = |s: Subscription| s.request(10);

    observer_1.all(Box::new(|&data: &i32| data > 0)).subscribe(
        Box::new(on_next),
        Box::new(on_error),
        Box::new(on_completed),
    )
}
