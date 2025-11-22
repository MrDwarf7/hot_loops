#![feature(vec_into_raw_parts)]
use anyhow::Result;
use std::sync::{Arc, atomic::AtomicUsize};

const BIG_NUM: usize = 100_000_000;
const CHUNK_SIZE: usize = 8192;

fn main() -> Result<()> {
    let num_cpus = num_cpus::get();

    // let buffer = Arc::new(parking_lot::Mutex::new(std::io::stdout()));

    let total_chunks = BIG_NUM.div_ceil(CHUNK_SIZE);
    println!("Total chunks: {}", total_chunks);

    // let stdout = std::io::stdout();

    let atomic = Arc::new(AtomicUsize::new(1));
    let global_cheksum = Arc::new(AtomicUsize::new(0));
    std::thread::scope(|s| {
        for _ in num_cpus.saturating_sub(1)..=num_cpus {
            let atomic = Arc::clone(&atomic);
            // let stdout = Arc::clone(&buffer);
            let global_cheksum = Arc::clone(&global_cheksum);

            s.spawn(move || {
                let mut local_buffer = Vec::with_capacity(CHUNK_SIZE * 16);
                let mut local_checksum = 0u64;

                loop {
                    let chunk_id = atomic.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    if chunk_id >= total_chunks {
                        break;
                    }

                    let start = chunk_id * CHUNK_SIZE + 1;
                    let end = std::cmp::min(start + CHUNK_SIZE - 1, BIG_NUM);
                    local_buffer.clear();

                    // fizz_buzz_batch(start, end - start + 1, &mut local_buffer);
                    fizz_buzz_batch_unsafe(start, end - start + 1, &mut local_buffer);

                    for &byte in &local_buffer {
                        local_checksum = local_checksum.wrapping_add(byte as u64);
                    }

                    ///////////////////////
                    // {
                    //     let mut handle = stdout.lock();
                    //     handle.write_all(&local_buffer).unwrap_or_else(|e| {
                    //         eprintln!("Error writing to stdout: {:?}", e);
                    //     });
                    // }
                }

                global_cheksum
                    .fetch_add(local_checksum as usize, std::sync::atomic::Ordering::SeqCst);
            });
        }
    });

    println!(
        "Checksum: {}",
        global_cheksum.load(std::sync::atomic::Ordering::SeqCst)
    );

    // buffer.lock().flush()?;
    Ok(())
}

#[allow(dead_code)]
#[inline]
fn fizz_buzz_batch(start: usize, count: usize, buffer: &mut Vec<u8>) {
    const FIZZBUZZ: &[u8] = b"FizzBuzz\n";
    const FIZZ: &[u8] = b"Fizz\n";
    const BUZZ: &[u8] = b"Buzz\n";

    let mut fizz = start % 3;
    let mut buzz = start % 5;

    for i in start..(start + count) {
        match (fizz, buzz) {
            (0, 0) => buffer.extend_from_slice(FIZZBUZZ),
            (0, _) => buffer.extend_from_slice(FIZZ),
            (_, 0) => buffer.extend_from_slice(BUZZ),
            (_, _) => buffer.extend_from_slice(format!("{}\n", i).as_bytes()),
        }

        fizz = if fizz == 2 { 0 } else { fizz + 1 };
        buzz = if buzz == 4 { 0 } else { buzz + 1 };
    }
}

#[inline]
fn fizz_buzz_batch_unsafe(start: usize, count: usize, buffer: &mut Vec<u8>) {
    const FIZZBUZZ: &[u8] = b"FizzBuzz\n";
    const FIZZ: &[u8] = b"Fizz\n";
    const BUZZ: &[u8] = b"Buzz\n";

    let mut fizz = start % 3;
    let mut buzz = start % 5;

    let estimated_size = count * 10; // generous 
    if buffer.capacity() < buffer.len() + estimated_size {
        buffer.reserve_exact(estimated_size);
    }

    unsafe {
        let buffer_ptr = buffer.as_mut_ptr();
        let mut ptr = buffer_ptr.add(buffer.len());
        let buffer_end = buffer_ptr.add(buffer.capacity());
        // let original_len = buffer.len();

        for i in start..(start + count) {
            let bytes_to_write = match (fizz, buzz) {
                (0, 0) => FIZZBUZZ,
                (0, _) => FIZZ,
                (_, 0) => BUZZ,
                (_, _) => {
                    let bytes_written = write_int_unsafe(i, ptr, buffer_end);
                    ptr = ptr.add(bytes_written);
                    *ptr = b'\n';
                    ptr = ptr.add(1);

                    fizz = if fizz == 2 { 0 } else { fizz + 1 };
                    buzz = if buzz == 4 { 0 } else { buzz + 1 };
                    continue;
                }
            };

            // Bounds check
            let bytes_needed = bytes_to_write.len();
            if ptr.add(bytes_needed) > buffer_end {
                // use safe if out of space
                let curr_len = ptr.offset_from(buffer.as_ptr()) as usize;
                buffer.set_len(curr_len);
                buffer.extend_from_slice(bytes_to_write);
                ptr = buffer.as_mut_ptr().add(buffer.len());
            } else {
                // use direct memory
                std::ptr::copy_nonoverlapping(bytes_to_write.as_ptr(), ptr, bytes_needed);

                ptr = ptr.add(bytes_needed);
            }

            fizz = if fizz == 2 { 0 } else { fizz + 1 };
            buzz = if buzz == 4 { 0 } else { buzz + 1 };
        }

        let final_len = ptr.offset_from(buffer.as_ptr()) as usize;
        buffer.set_len(final_len);
    }
}

#[inline]
fn write_int_unsafe(mut num: usize, ptr: *mut u8, buffer_end: *const u8) -> usize {
    unsafe {
        if num == 0 {
            if ptr.add(1) >= buffer_end as *mut u8 {
                *ptr = b'0';
                return 1;
            }
            return 0;
        }

        // count digits
        let mut temp = num;
        let mut digit_count = 0;
        while temp > 0 {
            temp /= 10;
            digit_count += 1;
        }

        if ptr.add(digit_count) > buffer_end as *mut u8 {
            return 0;
        }

        // write backwards
        let mut current_ptr = ptr.add(digit_count - 1);
        while num > 0 {
            *current_ptr = (num % 10) as u8 + b'0';
            num /= 10;
            // Use wrapping????
            current_ptr = current_ptr.sub(1);
        }

        digit_count
    }
}
