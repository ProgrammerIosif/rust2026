use std::{fs, io};

fn p1() -> Result<(), io::Error> {
    let s = fs::read_to_string("file.txt")?;

    let mut max_chars = 0;
    let mut max_line = "".to_string();
    for l in s.lines() {
        if l.chars().count() > max_chars {
            max_chars = l.chars().count();
            max_line = l.to_string();
        }
    }
    println!("{max_line}");

    let mut max_bytes = 0;
    max_line = "".to_string();
    for l in s.lines() {
        if l.len() > max_bytes {
            max_bytes = l.len();
            max_line = l.to_string();
        }
    }
    println!("{max_line}");
    Ok(())
}

#[derive(Debug)]
enum Error {
    NotAsciiChar,
}

fn p2(input: &str) -> Result<String, Error> {
    if input.chars().any(|c| !c.is_ascii_alphabetic()) {
        return Err(Error::NotAsciiChar);
    }
    Ok(input
        .chars()
        .map(|c| {
            if ('a'..='m').contains(&c) || ('A'..='M').contains(&c) {
                (c as u8 + 13) as char
            } else {
                (c as u8 - 13) as char
            }
        })
        .collect())
}

fn p3() -> Result<(), io::Error> {
    let list = [
        ("pentru", "ptr"),
        ("pentru", "pt"),
        ("domnul", "dl"),
        ("doamna", "dna"),
    ];

    let s = fs::read_to_string("p3")?;
    let res: Vec<_> = s
        .split_whitespace()
        .map(|w| {
            for i in list {
                if i.1 == w {
                    return i.0;
                }
            }
            w
        })
        .collect();
    let res = res.join(" ");
    println!("{res}");
    Ok(())
}

fn p4() -> Result<(), io::Error> {
    let s = fs::read_to_string("/etc/hosts")?;
    for l in s.lines() {
        if let Some(c) = l.chars().next()
            && c != '#'
            && let Some(w1) = l.split_whitespace().next()
            && let Some(w2) = l.split_whitespace().nth(1)
        {
            println!("{} => {}", w1, w2);
        }
    }
    Ok(())
}

fn main() {
    let _ = p1();

    match p2("HElLo") {
        Ok(res) => println!("{res}"),
        Err(e) => println!("{:?}", e),
    }
    match p2("ih29-3") {
        Ok(res) => println!("{res}"),
        Err(e) => println!("{:?}", e),
    }

    let _ = p3();
    let _ = p4();
}
