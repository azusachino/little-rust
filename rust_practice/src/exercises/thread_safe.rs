#![allow(dead_code)]

mod tests {
    // 此消息用于发送到与「主组件」并行运行的其他组件。
    enum WorkMsg {
        Work(u8),
        Exit,
    }

    // 此消息用于从并行运行的其他组件 发送回「主组件」。
    enum ResultMsg {
        Result(u8),
        Exited,
    }

    #[test]
    fn main() {
        use crossbeam_channel::unbounded;
        use std::thread;

        let (work_sender, work_receiver) = unbounded();
        let (result_sender, result_receiver) = unbounded();

        // 生成子线程用于执行另一个并行组件
        let _ = thread::spawn(move || loop {
            // 接收并处理消息，直到收到 exit 消息
            match work_receiver.recv() {
                Ok(WorkMsg::Work(num)) => {
                    // 执行一些工作，并且发送消息给 Result 队列
                    let _ = result_sender.send(ResultMsg::Result(num));
                }
                Ok(WorkMsg::Exit) => {
                    // 发送 exit 确认消息
                    let _ = result_sender.send(ResultMsg::Exited);
                    break;
                }
                _ => panic!("Error receiving a WorkMsg."),
            }
        });

        let _ = work_sender.send(WorkMsg::Work(0));
        let _ = work_sender.send(WorkMsg::Work(1));
        let _ = work_sender.send(WorkMsg::Exit);

        // worker执行计数
        let mut counter = 0;

        loop {
            match result_receiver.recv() {
                Ok(ResultMsg::Result(num)) => {
                    // 断言确保接收和发送的顺序是一致的
                    assert_eq!(num, counter);
                    counter += 1;
                }
                Ok(ResultMsg::Exited) => {
                    // 断言确保在接收两条工作消息之后收到退出消息
                    assert_eq!(2, counter);
                    break;
                }
                _ => panic!("Error receiving a ResultMsg."),
            }
        }
    }

    #[test]
    fn test2() {
        use std::{
            sync::{Arc, Mutex},
            thread,
        };

        let v = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
        for i in 0..3 {
            let clone_v = v.clone();
            thread::spawn(move || {
                let mut tmp = clone_v.lock().unwrap();
                tmp.push(i);
            });
        }
    }
}

mod scopes {
    use crossbeam; // 0.6.0
    use std::{thread, time::Duration};

    fn scoped() {
        let vec = vec![1, 2, 3, 4, 5];

        crossbeam::scope(|scope| {
            for e in &vec {
                scope.spawn(move |_| {
                    println!("{:?}", e);
                });
            }
        })
        .expect("A child thread panicked");

        println!("{:?}", vec);
    }

    fn scope_thread() {
        let mut vec = vec![1, 2, 3, 4, 5];

        crossbeam::scope(|scope| {
            for e in &mut vec {
                scope.spawn(move |_| {
                    thread::sleep(Duration::from_secs(1));
                    *e += 1;
                });
            }
        })
        .expect("A child thread panicked");

        println!("{:?}", vec);
    }
}

mod atomics {

    #[test]
    fn spin_lock() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        use std::{thread, time};

        let spin_lock = Arc::new(AtomicUsize::new(1));

        let sp_clone = spin_lock.clone();
        let thd = thread::spawn(move || {
            // use SeqCst to ensure execute order
            sp_clone.store(1, Ordering::SeqCst);
            let t = time::Duration::from_secs(3);
            thread::sleep(t);
            sp_clone.store(0, Ordering::SeqCst);
        });

        // spin wait
        while spin_lock.load(Ordering::SeqCst) != 0 {}

        if let Err(e) = thd.join() {
            println!("thread had an error: {:?}", e);
        }
    }

    use core::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    struct LightLock(AtomicBool);

    impl LightLock {
        pub fn new() -> LightLock {
            LightLock(AtomicBool::new(false))
        }

        pub fn try_lock<'a>(&'a self) -> Option<LightGuard<'a>> {
            let was_locked = self.0.swap(true, Ordering::Acquire);
            if was_locked {
                None
            } else {
                Some(LightGuard { lock: self })
            }
        }
    }

    struct LightGuard<'a> {
        lock: &'a LightLock,
    }

    impl<'a> Drop for LightGuard<'a> {
        fn drop(&mut self) {
            self.lock.0.store(false, Ordering::Release);
        }
    }
}
