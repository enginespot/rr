use std::error::Error;

use rr::{Publisher, RRResult};

#[test]
fn test_merge() -> RRResult<()> {
    let map_1: Vec<i32> = vec![1, 2, 3, 4];
    let on_next = |v: i32| {
        println!("{:?}", v);
        Ok(())
    };
    let on_error = |_e: Box<dyn Error + Send>| {
        println!("error occur");
        Ok(())
    };
    let on_completed = || {
        println!("merge done");
        Ok(())
    };

    let observer_1 = Publisher::from_iterable(map_1);

    let map_2: Vec<i32> = vec![5, 6, 7, 8];

    let observer_2 = Publisher::from_iterable(map_2);

    let observer_merge = Publisher::merge(vec![observer_1, observer_2]);

    observer_merge
        .map(Box::new(|data: i32| Ok(data + 100)))
        .subscribe(
            Box::new(on_next),
            Box::new(on_error),
            Box::new(on_completed),
        )
        .map(|_| {})
}

#[test]
fn test_merge_with() -> RRResult<()> {
    let map_1: Vec<i32> = vec![1, 2, 3, 4];
    let on_next = |v: i32| {
        println!("{:?}", v);
        Ok(())
    };
    let on_error = |_e: Box<dyn Error + Send>| {
        println!("error occur");
        Ok(())
    };
    let on_completed = || {
        println!("merge done");
        Ok(())
    };

    let observer_1 = Publisher::from_iterable(map_1);

    let map_2: Vec<i32> = vec![5, 6, 7, 8];

    let observer_2 = Publisher::from_iterable(map_2);

    let observer_merge = observer_1.merge_with(observer_2);
    observer_merge
        .map(Box::new(|data: i32| Ok(data + 100)))
        .subscribe(
            Box::new(on_next),
            Box::new(on_error),
            Box::new(on_completed),
        )
        .map(|_| {})
}
