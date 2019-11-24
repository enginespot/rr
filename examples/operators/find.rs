use std::error::Error;

use rr::{Publisher, RRResult, Subscription};

pub fn run() -> RRResult<Subscription> {
    let map_1: Vec<i32> = vec![1, 2, 3, 4];

    let observer_1 = Publisher::from_iterable(map_1);

    let on_next = |v: i32| {
        println!("{:?}", v);
        Ok(())
    };
    let on_error = |e: Box<dyn Error + Send>| {
        println!("{:?}", e);
        Ok(())
    };
    let on_completed = || {
        println!("find done");
        Ok(())
    };

    observer_1.find(Box::new(|&data| Ok(data > 1))).subscribe(
        Box::new(on_next),
        Box::new(on_error),
        Box::new(on_completed),
    )
}
