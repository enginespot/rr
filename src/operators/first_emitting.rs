use crate::Publisher;
impl<'a, T: 'a + Send> Publisher<'a, T> {
    //    pub fn first_emitting(self, emitters: Vec<Publisher<'a, T>>) -> Publisher<'a, T> {
    //        let func = move |mut subscriber: Box<Subscriber<'a, T>>| {
    //            //            //            存放第一个发送成功的
    //            //            let (mut who, find) = (None, Arc::new(AtomicBool::new(false)));
    //            //            //存放未发送成功的 就放到一个Vec中把
    //            //
    //            //            let mut started_publisher: Vec<(usize, Arc<Mutex<Publisher<'a, T>>>)> = Vec::new();
    //            //            let unsubscribe = |except: usize| {
    //            //                for (index, ob) in started_publisher.into_iter().enumerate() {
    //            //                    if ob.0 != except {
    //            //                        let ob = ob.1.lock().unwrap();
    //            //                        ob.cancel();
    //            //                    }
    //            //                }
    //            //            };
    //            //
    //            //            for (index, ob) in emitters.into_iter().enumerate() {
    //            //                let is_find = find.load(Ordering::Acquire);
    //            //                let ob = Arc::new(Mutex::new(ob));
    //            //                started_publisher.push((index, ob.clone()));
    //            //
    //            //                if is_find {
    //            //                    //                    需要通知其它的Observable全部取消订阅
    //            //                    break;
    //            //                } else {
    //            //                    let on_completed = move |index: usize| {
    //            //                        //                        move || {
    //            //                        //                            let mut o_c = o_c.lock().unwrap();
    //            //                        //                            o_c.on_completed();
    //            //                        //                        }
    //            //                    };
    //            //
    //            //                    let on_next = move |index: usize| {
    //            //                        //                        let success = find.compare_exchange(
    //            //                        //                            false,
    //            //                        //                            true,
    //            //                        //                            Ordering::Acquire,
    //            //                        //                            Ordering::Relaxed,
    //            //                        //                        );
    //            //                        //                        if success {
    //            //                        //                            //                    需要通知其它的Observable全部取消订阅
    //            //                        //                            who = Some(index);
    //            //                        //                        }
    //            //                    };
    //            //
    //            //                    let on_error = move |index: usize| {
    //            //                        //
    //            //                        move |mut data: Box<dyn Error >| {
    //            //                            //                    需要通知其它的Observable全部取消订阅
    //            //                            //                                                    subscriber.on_error(data);
    //            //                        };
    //            //                    };
    //            //
    //            //                    let ob = ob.clone();
    //            //                    let ob = ob.lock().unwrap();
    //            //                    ob.subscribe(
    //            //                        Box::new(on_next(index)),
    //            //                        Box::new(on_error(index)),
    //            //                        Box::new(on_completed(index)),
    //            //                    );
    //            //                }
    //            //            }
    //        };
    //        Publisher::create(Box::new(func))
    //    }
}
