use std::error::Error;

use rr::{Publisher, Subscriber, Subscription};

use rr::RRResult;

pub fn run() -> RRResult<Subscription> {
    let on_next = |v: i32| {
        println!("create next {:?}", v);
        Ok(())
    };
    let on_error = |_e: Box<dyn Error + Send>| {
        println!("error occur");
        Ok(())
    };
    let on_completed = || {
        println!("done");
        Ok(())
    };

    Publisher::create(Box::new(|x: Box<Subscriber<i32>>| {
        x.on_next(1)?;
        x.on_completed()?;
        let subscription = Subscription::new();
        Ok(subscription)
    }))
    .subscribe(
        Box::new(on_next),
        Box::new(on_error),
        Box::new(on_completed),
    )
}

pub fn main() {}
