<div align="center">

# Multiplication Persistance

Multiply all the digits of a number n by each other, repeating with the product until a single digit is obtained. The number of steps required is known as the multiplicative persistence, and the final digit obtained is called the multiplicative digital root of n.

</div>

## Specifications

- **Rust Language** is used to get the best performance and memory safety.
- **Concurrency and Multithreading** is used to speed up the process of testing the numbers.
- **Big Integers** are used to test for numbers greater than `10`<sup>`20,000`</sup>.
- **MPSC (Multi-producer, single-consumer)** is used to communicate between the main thread and the child threads.
- **Custom Algorithm** is used to find the next best number to be tested for multiplicative persistance.

## Motivation

> This repo is an attempt to create an algorithm that will test for numbers greater than `10`<sup>`20,000`</sup> [^1] for the multiplicative persistance and find a lest number with multiplicative persistance greater than `11`[^2].

[^1]: Numbers less than `10`<sup>`20,000`</sup> has already been tested by many and there is no number less than that has multiplicative persistance of greater than `11`.
[^2]: `11` is the largest known multiplicative persistance known till date.

## Algorithm

This Program is designed in a way to utilize concurrency and multithreading in rust programming language to check for more no.of numbers in a faster way.

This algorithm consist of three helper function and one main function.

**Auxiliary Functions:**

- `to_digits(num: &mut Integer,  original_vec: &mut Vec<u8>)`
- `mul_per_vec(num: &mut Vec<u8>, steps: &mut u8) -> u8`
- `best_next(num: &mut Vec<u8>, i: usize)`

### Main Function

This is the function in with the child threads are spawned and managed. This is the entry/controller of the entire code base.

### `to_digits()`

Converts the `Integer` type to `Vec<u8>` type. That is, it converts the number to a vector of digits.

| Parameter      | Type           | Description                                     |
| -------------- | -------------- | ----------------------------------------------- |
| `num`          | `&mut Integer` | The number to be converted to vector of digits  |
| `original_vec` | `&mut Vec<u8>` | The vector to which the digits are to be stored |

This mutates the `original_vec` and returns nothing.

**Implementation:**

```rs
/// Convert the original number into digits and replace the original vec with new one
fn to_digits(num: &mut Integer,  original_vec: &mut Vec<u8>) {
    let base = 10;
    let mut digits = Vec::new();
    while *num > 0 {
        let digit = num.mod_u(base);
        digits.push(digit as u8);
        *num /= base;
    }
    *original_vec = digits;
}
```

### `mul_per_vec()`

Calculates the multiplicative persistance of a number represented as a vector of digits.

| Parameter | Type           | Description                                     |
| --------- | -------------- | ----------------------------------------------- |
| `num`     | `&mut Vec<u8>` | The number to be converted to vector of digits  |
| `steps`   | `&mut u8`      | The vector to which the digits are to be stored |

This mutates the `num` and returns the multiplicative persistance of the number. The `steps` parameter is used to keep track of the number of steps required to reach the multiplicative digital root. This is used to find the multiplicative persistance of the number.

**Implementation:**

```rs
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
```

### `best_next()`

This function is used to find the next best number to be tested for multiplicative persistance. This is done by incrementing the number by one and replacing the digits that are in `IGNORE` array with the next best digit.

This function basically is a custom way to increment the number which are represented as vector of digits. Instead of incrementing the number from the last to first digit, this function increments the number from the first to last digit and each time it increments the number, it replaces the digits that are in `IGNORE` array with the next best digit.

> The `IGNORE` array is used to ignore the digits that are not required to be tested for multiplicative persistance. This is done to reduce the number of numbers to be tested. This is done by replacing the digits in the number with the next best digit.

| Parameter | Type           | Description                                    |
| --------- | -------------- | ---------------------------------------------- |
| `num`     | `&mut Vec<u8>` | The number to be converted to vector of digits |
| `i`       | `usize`        | The index of the digit to be incremented       |

This mutates the `num` and returns nothing. The `i` parameter is used to keep track of the index of the digit to be incremented.

**Implementation:**

```rs
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
```

### But why to ignore some digits?

The digits that are ignored are the digits that are not required to be tested for multiplicative persistance. It is very obvious that the digits `0`, `1` and `5` are not required to be tested for multiplicative persistance.

| Digit | Reason                                                                                                                                                                          |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `0`   | `0` is the additive identity. So, the multiplicative persistance of number with `0` is `1`.                                                                                     |
| `1`   | `1` is the multiplicative identity. So, it is not significant for multiplicative persistance                                                                                    |
| `5`   | if we multiply any number with `5`, the last digit of the result will be either `0` or `5`. So, it is will make the multiplicative persistance to very less, around `2` or `3`. |

## Usage

```sh
cargo run --release
```

## Bottlenecks

Even though this algorithm is faster than the naive approach, it still has some bottlenecks. The implementation is not fully optimized and there are some places where the performance can be improved. For example, current implementation is faster and runs smoothly on CPU but the memory (RAM) usage is very high and it increases over no.of iterations.

## Contributing

Any kind of contribution is welcome. Feel free to open an issue or a pull request.
Any can contribute to this project by improving the algorithm or by improving the code quality. Feel free to discuss on the issues.

## Reference

1. *https://en.wikipedia.org/wiki/Persistence_of_a_number*
2. *https://mathworld.wolfram.com/MultiplicativePersistence.html (**Note:** has outdated information)*

---

<div align="center">

Made with ❤️ in India by [Rajaniraiyn](https://dub.sh/raja-portfolio).

</div>
