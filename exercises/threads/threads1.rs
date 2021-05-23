// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    // Better to be explicit about Arc::clone versus .clone()
    // See note at end of https://doc.rust-lang.org/book/ch15-04-rc.html#using-rct-to-share-data
    let status_shared = Arc::clone(&status);
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status_shared.lock().unwrap().jobs_completed += 1;
        }
    });
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
/*
question - is Mutex required even if only one writer?
answer - Yes, the rules are:
    - one mutable reference
    or
    - multiple immutable references

    Example of conflict:
    let mut x = 1;
    let y = &x; // borrowed x
    x = 3; // can't assign x
    let z = *y; // necessary so borrowed line isn't optimized away

    So once data become mutable, can't share read-only references.
*/
