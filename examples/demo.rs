use crate::operators::{
    all, do_on_next, find, flat_map, flatten, map, merge, observe_on_new_thread,
};
use rr::{RRResult, Subscription};

mod create;
mod operators;
mod util;

fn main() {
    let _ = demo();
}
fn demo() -> RRResult<Subscription> {
    let _ = create::run()?;
    let _ = all::run()?;
    let _ = find::run()?;
    let _ = flat_map::run()?;
    let _ = flatten::run()?;
    let _ = map::run()?;
    let _ = merge::run()?;
    let _ = do_on_next::run()?;
    observe_on_new_thread::run()
}
