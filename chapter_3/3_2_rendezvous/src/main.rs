use std::sync::{Arc, Semaphore};
use std::thread::Thread;

// Our functions. a1 and b1, and a2 and b2
// can happen concurrently, but a1 must happen
// before b2 and b1 must happen before a2
fn statement_a1() { println!("a1"); }
fn statement_a2() { println!("a2"); }

fn statement_b1() { println!("b1"); }
fn statement_b2() { println!("b2"); }

#[allow(unstable)]
fn main() {

    // We use 2 semaphores here: sem_a to signal that
    // statement_a2 can safely run, and sem_b to signal
    // that statement_b2 can safely run
    let sem_a = Arc::new(Semaphore::new(0));
    let sem_b = Arc::new(Semaphore::new(0));

    let s_a1 = sem_a.clone();
    let _ = Thread::scoped(move ||{
        statement_a1();
        s_a1.release();
    });

    let s_b2 = sem_b.clone();
    let _t = Thread::scoped(move ||{
        s_b2.acquire();
        statement_a2();
    });

    let s_b1 = sem_b.clone();
    let _ = Thread::scoped(move ||{
        statement_b1();
        s_b1.release();
    });

    let s_a2 = sem_a.clone();
    let _ = Thread::scoped(move ||{
        s_a2.acquire();
        statement_b2();
    });
}