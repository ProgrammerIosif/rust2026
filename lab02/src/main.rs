fn add_chars_n(s: String, c: char, n: u32) -> String {
    let mut s = s;
    for _ in 0..n {
        s.push(c);
    }
    s
}

fn add_chars_n_ref(s: &mut String, c: char, n: u32) {
    for _ in 0..n {
        s.push(c);
    }
}

fn add_space(s: String, n: u32) -> String {
    add_chars_n(s, ' ', n)
}

fn add_str(s: String, to_add: &str) -> String {
    s + to_add
}

fn add_integer(s: String, n: u32) -> String {
    let mut digit_count = 0;
    let mut num = "".to_string();
    let mut n = n;
    while n != 0 {
        num.push(((n % 10) as u8 + b'0') as char);
        digit_count += 1;
        n /= 10;
        if digit_count % 3 == 0 && n != 0 {
            num.push('_');
        }
    }
    s + &num.chars().rev().collect::<String>()
}

fn add_float(s: String, number: f32) -> String {
    let mut fract = number.fract() * 10.;
    let mut num = "".to_string();
    while fract > 0. {
        num.push((fract as u8 + b'0') as char);
        fract = fract.fract() * 10.;
    }
    add_integer(s, number as u32) + "." + &num
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    println!("{}", s);

    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n_ref(&mut s, c, 26 - i);

        i += 1;
    }

    println!("{}", s);

    let mut s2 = "Most".to_string();
    s2 = add_space(s2, "downloaded".len() as u32);
    s2 = add_str(s2, "crate");
    s2 = add_space(s2, "has".len() as u32);
    s2 = add_integer(s2, 306437968);
    s2 = add_space(s2, "downloads".len() as u32);
    s2 = add_str(s2, "and");
    s2 = add_space(s2, "the".len() as u32);
    s2 = add_str(s2, "latest");
    s2 = add_space(s2, "version".len() as u32);
    s2 = add_str(s2, "is");
    println!("{s2}");

    let mut s3 = "".to_string();
    s3 = add_space(s3, "Most".len() as u32);
    s3 = add_str(s3, "downloaded");
    s3 = add_space(s3, "crate".len() as u32);
    s3 = add_str(s3, "has");
    s3 = add_space(s3, "306_437_968".len() as u32);
    s3 = add_str(s3, "downloads");
    s3 = add_space(s3, "and".len() as u32);
    s3 = add_str(s3, "the");
    s3 = add_space(s3, "latest".len() as u32);
    s3 = add_str(s3, "version");
    s3 = add_space(s3, "is".len() as u32);
    s3 = add_float(s3, 2.083);

    println!("{s3}");
}
