// use people::Person;

// pub fn prev_main() -> Result<()> {
//     #[inline]
//     fn is_even(value: usize) -> bool {
//         value % 2 != 0
//     }
//
//     #[inline]
//     fn go_hard(n: usize) -> usize {
//         n * n
//     }
//
//     let mut counter = 0;
//
//     let mut buf = String::new();
//
//     let stdout = std::io::stdout();
//     let mut handle = stdout.lock();
//
//     while counter < BIG_NUM {
//         if is_even(counter) {
//             let v = go_hard(counter);
//             let msg = format!("{} is even, go hard: {}\n", counter, v);
//             buf.push_str(&msg);
//         }
//         counter += 1;
//
//         if counter >= BIG_NUM {
//             break;
//         }
//     }
//
//     // Write the buffer to stdout
//     handle.write_all(buf.as_bytes())?;
//     handle.flush()?;
//
//     Ok(())
// }
