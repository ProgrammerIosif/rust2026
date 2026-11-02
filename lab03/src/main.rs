fn is_prime(x: u16) -> bool {
    if x == 0 || x == 1 {
        false
    } else {
        let mut n = 0;
        for i in 2..=x / 2 {
            if x.is_multiple_of(i) {
                n += 1;
            }
        }
        n == 0
    }
}

fn next_prime(x: u16) -> Option<u16> {
    let mut x = x;
    while x != u16::MAX {
        x += 1;
        if is_prime(x) {
            return Some(x);
        }
    }
    None
}

// fn checked_addition(x: u32, y: u32) -> u32 {
//     match x.checked_add(y) {
//         Some(res) => res,
//         None => panic!("addition overflowed")
//     }
// }
//
// fn checked_multiplication(x: u32, y: u32) -> u32 {
//     match x.checked_mul(y) {
//         Some(res) => res,
//         None => panic!("multiplication overflowed")
//     }
// }

#[derive(Debug)]
enum OverflowError {
    MultiplicationOverflow,
    AdditionOverflow,
}

fn checked_addition(x: u32, y: u32) -> Result<u32, OverflowError> {
    match x.checked_add(y) {
        Some(res) => Ok(res),
        None => Err(OverflowError::AdditionOverflow),
    }
}

fn checked_multiplication(x: u32, y: u32) -> Result<u32, OverflowError> {
    match x.checked_mul(y) {
        Some(res) => Ok(res),
        None => Err(OverflowError::MultiplicationOverflow),
    }
}

fn calculate(a: u32, b: u32, c: u32) -> Result<u32, OverflowError> {
    let product = checked_multiplication(a, b)?;
    let sum = checked_addition(product, c)?;

    Ok(sum)
}

fn main() {
    let mut x: u16 = 1;
    while let Some(y) = next_prime(x) {
        print!("{y} ");
        x = y;
    }

    match calculate(u32::MAX, 2, 3) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {:?}", error),
    }
}
