use std::sync::{Arc, Semaphore};

fn main() {

    // Raw semaphores aren't shareable in
    // procs, so they need to be wrapped in
    // an Arc pointer
    let sem = Arc::new(Semaphore::new(0));

    // Pointers must be cloned before they can
    // be used in a Proc because they are moved
    let sb = sem.clone();
    spawn(proc(){

        // The semaphore will be -1, and can this proc
        // can only continue once "Task A" has
        // been printed and the semaphore has been
        // released
        sb.acquire();
        println!("Task B");
    });

    let sa = sem.clone();
    spawn(proc(){
        println!("Task A");

        // "Task B" can now be printed, as the semaphore
        // value is now 0
        sa.release();
    });
}
