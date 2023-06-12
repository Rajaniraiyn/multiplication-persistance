use rug::Integer;
use std::sync::mpsc::channel;
use std::thread;

const RECORD: u8 = 11;

/// Convert the original number into digits and replace the original vec with new one
fn to_digits(num: &mut Integer, original_vec: &mut Vec<u8>) {
    let base = 10;
    let mut digits = Vec::new();
    while *num > 0 {
        let digit = num.mod_u(base);
        digits.push(digit as u8);
        *num /= base;
    }
    *original_vec = digits;
}

/// Multiply the digits of the number and return the number of steps
fn mul_per_vec(num: &mut Vec<u8>, steps: &mut u8) -> u8 {
    if num.len() <= 1 {
        return *steps;
    }

    let mut mul: Integer = Integer::from(1);

    for digit in num.to_owned() {
        mul *= digit;
    }

    *steps += 1;

    to_digits(&mut mul, num);

    return mul_per_vec(num, steps);
}

/// The digits that are to be ignored as they are not required to be tested
const IGNORE: [u8; 3] = [0, 1, 5];

/// this increments the vec by considering it as number without IGNORE
/// increments the array from the beginning
fn best_next(num: &mut Vec<u8>, i: usize) {
    num[i] += 1;

    if num[i] > 9 {
        num[i] = 2;
        if i + 1 >= num.len() {
            num.push(2);
        } else {
            best_next(num, i + 1);
        }
    }

    while IGNORE.contains(&num[i]) {
        num[i] += 1;
    }
}

fn main() {
    let mut threads = vec![];
    let (tx, rx) = channel();

    for i in &[3, 4, 6, 7, 8, 9] {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            println!("Running Thread {}", i);
            let mut num = vec![*i; 20_000]; // 20,000 digits is the lower bound
            let mut count: u128 = 0;
            loop {
                let steps = mul_per_vec(&mut num.clone(), &mut 0);
                if steps > RECORD {
                    num.sort();
                    println!(
                        "\nFound {} in with {} steps",
                        num.iter().map(|&x| x.to_string()).collect::<String>(),
                        steps
                    );
                    break;
                }
                best_next(&mut num, 0);
                count += 1;
                tx.send((i, count)).unwrap();
            }
        });

        threads.push(handle);
    }

    drop(tx);

    let mut counts = vec![0; 10];

    for (i, count) in rx {
        counts[*i as usize] = count;
        print!("{}\r {:?}", 8u8 as char, counts); // 8u8 is backspace
    }

    for handle in threads {
        handle.join().unwrap();
    }
}
