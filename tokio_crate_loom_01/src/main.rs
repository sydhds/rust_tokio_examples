use loom::sync::Arc;
use loom::sync::Mutex;
use loom::sync::atomic::AtomicUsize;
use loom::sync::atomic::Ordering;
use loom::thread;

// RUSTFLAGS="--cfg loom" cargo test buggy_concurrent_inc --release

#[test]
#[should_panic]
fn buggy_concurrent_inc() {

    // Can fail because: something can happen between load and store
    // 1- thread 1: num.load(...) then parked by OS (num = 0)
    // 2- thead 2: num.load(...) then store (num = 1)
    // 3- thread 1: num.store(...) (num = 1)

    loom::model(|| {

        let num = Arc::new(AtomicUsize::new(0));
        let ths: Vec<_> = (0..2).map(|_| {
            let num = num.clone();
            thread::spawn(move || {
                let curr = num.load(Ordering::Acquire);
                num.store(curr + 1, Ordering::Release);
            })
        }).collect();

        for th in ths {
            th.join().unwrap();
        }

        assert_eq!(2, num.load(Ordering::Relaxed));
    });
}

#[test]
fn buggy_concurrent_inc_fixed() {

    loom::model(|| {

        let num = Arc::new(AtomicUsize::new(0));
        let ths: Vec<_> = (0..2).map(|_| {
            let num = num.clone();
            thread::spawn(move || {
                num.fetch_add(1, Ordering::AcqRel);
            })
        }).collect();

        for th in ths {
            th.join().unwrap();
        }

        // assert_eq!(2, num.load(Ordering::SeqCst));
        assert_eq!(2, num.load(Ordering::Relaxed));
    });
}

#[test]
#[should_panic]
fn concurrent_logic() {

    loom::model(|| {
        let v1 = Arc::new(AtomicUsize::new(0));
        let v2 = v1.clone();

        thread::spawn(move || {
            v1.store(1, Ordering::SeqCst);
        });

        // Hint: can be 0 or 1
        //       v2.load(..) can happen after or before v1.store(...)
        assert_eq!(0, v2.load(Ordering::SeqCst));
    });
}

#[test]
fn concurrent_logic_fixed() {

    loom::model(|| {
        let v1 = Arc::new(AtomicUsize::new(0));
        let v2 = v1.clone();

        let t = thread::spawn(move || {
            v1.store(1, Ordering::SeqCst);
        });

        t.join().unwrap();
        assert_eq!(1, v2.load(Ordering::SeqCst));
    });
}

fn main() {
    todo!()
}

