use std::sync::{Arc, Semaphore};
use std::thread::Thread;

#[allow(unstable)]
fn main() {

    // Raw semaphores aren't shareable in
    // procs, so they need to be wrapped in
    // an Arc pointer
    let sem = Arc::new(Semaphore::new(0));

    // Pointers must be cloned before they can
    // be used in a Thread because they are moved
    let sb = sem.clone();
    let _ta = Thread::scoped(move ||{

        // The semaphore will be -1, and can this proc
        // can only continue once "Task A" has
        // been printed and the semaphore has been
        // released
        sb.acquire();
        println!("Task B");
    });

    let sa = sem.clone();
    let _tb = Thread::scoped(move ||{
        println!("Task A");

        // "Task B" can now be printed, as the semaphore
        // value is now 0
        sa.release();
    });

    
}