use std::error::Error;

use rr::{Publisher, RRResult};

#[test]
fn test_take() -> RRResult<()> {
    let map_1: Vec<i32> = vec![1, 2, 3, 4];
    let observer_1 = Publisher::from_iterable(map_1);

    let on_next = |v: i32| {
        println!("take next {:?}", v);
        Ok(())
    };
    let on_error = |_e: Box<dyn Error + Send>| {
        println!("take error occur");
        Ok(())
    };
    let on_completed = || {
        println!("take done");
        Ok(())
    };

    observer_1
        .take(2)
        .subscribe(
            Box::new(on_next),
            Box::new(on_error),
            Box::new(on_completed),
        )
        .map(|_| {})
}

#[test]
fn test_merge_with() {}
