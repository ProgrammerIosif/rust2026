fn is_prime(x: u32) -> bool {
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

fn is_coprime(x: u32, y: u32) -> bool {
    if x == 1 || y == 1 {
        true
    } else if x == 0 || y == 0 {
        false
    } else {
        let mut x = x;
        let mut y = y;
        while x != y {
            if x > y {
                x -= y;
            } else {
                y -= x;
            }
        }
        x == 1
    }
}

fn beer_song(x: u32) {
    let end = if x > 1 { "s" } else { "" };
    println!(
        "
{x} bottle{end} of beer on the wall,
{x} bottle{end} of beer.
Take one down, pass it around,
{} bottle{} of beer on the wall.",
        if x > 1 {
            (x - 1).to_string()
        } else {
            "No".to_string()
        },
        if x == 2 { "" } else { "s" }
    );
}

fn main() {
    // 1
    for i in 0..=100 {
        if is_prime(i) {
            print!("{i} ");
        }
    }

    // 2
    for i in 0..=100 {
        for j in 0..=100 {
            if is_coprime(i, j) {
                print!("{i} {j}   ");
            }
        }
        println!();
    }

    // 3
    for i in (1..100).rev() {
        beer_song(i);
    }
}
