use rr::{Publisher, RRResult};
use std::error::Error;
mod operations;

#[test]
fn hello_vec_list_1() -> RRResult<()> {
    let xs_map: Vec<i32> = vec![1, 2, 3, 4];
    let on_next = |v: i32| {
        println!("{:?}", v);
        println!("next occur");
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

    Publisher::from_iterable(xs_map)
        .map(Box::new(|data: i32| Ok(data + 100)))
        .subscribe(
            Box::new(on_next),
            Box::new(on_error),
            Box::new(on_completed),
        )
        .map(|_| ())
}

#[test]
fn hello_vec_list_2() -> RRResult<()> {
    let list: Vec<i32> = vec![1, 2, 3, 4];

    let on_next = |v: i32| {
        println!("{:?}", v);
        println!("next occur");
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

    Publisher::from_iterable(list)
        .subscribe(
            Box::new(on_next),
            Box::new(on_error),
            Box::new(on_completed),
        )
        .map(|_| ())
}
