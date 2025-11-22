use std::{
    io::Write,
    sync::{Arc, atomic::AtomicUsize},
};

use anyhow::Result;

const BIG_NUM: usize = 100;

fn main() -> Result<()> {
    let atomic = AtomicUsize::new(1);
    let buffer = parking_lot::RwLock::new(std::io::stdout());

    let num_cpus = num_cpus::get();
    println!("Number of threads: {}", num_cpus);

    std::thread::scope(|s| {
        let handles = Arc::new(parking_lot::Mutex::new(Vec::new()));
        for t in 0..=num_cpus {
            println!("Spawning thread {}", t);
            let handles = Arc::clone(&handles);

            while atomic.load(std::sync::atomic::Ordering::SeqCst) <= BIG_NUM {
                if atomic.load(std::sync::atomic::Ordering::SeqCst) == BIG_NUM {
                    break;
                }
                let h = s.spawn(|| {
                    fizz_buzz(&atomic, &mut buffer.write().lock())?;
                    Ok::<(), anyhow::Error>(())
                });
                let mut handles = handles.lock();
                handles.push(h);
            }
        }

        s.spawn(move || {
            let mut handles = handles.lock();

            for handle in handles.drain(..) {
                if let Err(e) = handle.join() {
                    eprintln!("Thread error: {:?}", e);
                }
            }
        });
    });

    buffer.write().flush()?;

    Ok(())
}

fn fizz_buzz<'a>(atomic: &'a AtomicUsize, buffer: &mut std::io::StdoutLock<'a>) -> Result<()> {
    // while atomic.load(std::sync::atomic::Ordering::SeqCst) < BIG_NUM {
    let i = atomic.load(std::sync::atomic::Ordering::SeqCst);

    match (i % 3, i % 5) {
        (0, 0) => buffer.write_all(b"FizzBuzz\n")?,
        (0, _) => buffer.write_all(b"Fizz\n")?,
        (_, 0) => buffer.write_all(b"Buzz\n")?,
        (_, _) => buffer.write_all(format!("{}\n", i).as_bytes())?,
    }

    buffer.write_all(format!("VALUE: {}\n", i).as_bytes())?;
    atomic.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    // }
    Ok(())
}
