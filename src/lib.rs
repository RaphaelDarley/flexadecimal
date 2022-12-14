use core::fmt;
use std::ops;

#[derive(Debug)]
pub struct Flexadecimal {
    inner: [u8; 255],
}

impl fmt::Display for Flexadecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "〈{}〉", String::from(self))
    }
}

impl ops::Add<Flexadecimal> for Flexadecimal {
    type Output = Flexadecimal;

    fn add(self, rhs: Flexadecimal) -> Self::Output {
        let mut acc = [0; 255];
        let mut carry: u8 = 0;
        let mut add: usize;

        for i in 0..255 {
            add = (self.inner[i] + rhs.inner[i] + carry) as usize;
            acc[i] = (add % (i + 2)) as u8;
            carry = (add / (i + 2)) as u8;
        }

        Flexadecimal { inner: acc }
    }
}

impl Flexadecimal {
    pub fn new() -> Flexadecimal {
        Flexadecimal { inner: [0; 255] }
    }

    fn set_col(&mut self, col_num: usize, val: u8) {
        self.inner[col_num - 1] = val;
    }
}

impl From<usize> for Flexadecimal {
    fn from(item: usize) -> Flexadecimal {
        let mut out = Flexadecimal::new();
        if item == 0 {
            return out;
        }
        if item == 1 {
            out.inner[0] = 1;
            return out;
        }
        let col = highest_fac(item);
        let col_val = factorial(col);
        out.set_col(col, (item / (col_val)) as u8);
        out + Flexadecimal::from(item % col_val)
    }
}

impl From<Flexadecimal> for usize {
    fn from(item: Flexadecimal) -> usize {
        let mut acc: usize = 0;
        for (i, v) in item.inner.into_iter().enumerate() {
            if v != 0 {
                acc += v as usize * factorial(i + 1);
            }
        }

        acc
    }
}

impl From<&str> for Flexadecimal {
    fn from(item: &str) -> Flexadecimal {
        let mut acc = Flexadecimal::new();
        for (i, c) in item.chars().into_iter().rev().enumerate() {
            let val = c.to_digit(16).unwrap() as usize;
            let col_num = i + 1;
            if val > col_num {
                panic!("Invalid Flexadecimal literal")
            }
            acc.set_col(col_num, val as u8)
        }
        acc
    }
}

impl From<&Flexadecimal> for String {
    fn from(item: &Flexadecimal) -> String {
        item.inner
            .into_iter()
            .rev()
            .skip_while(|d| *d == 0)
            .map(|d| char::from_digit(d as u32, 16).unwrap())
            .collect()
    }
}

impl From<Flexadecimal> for String {
    fn from(item: Flexadecimal) -> String {
        item.inner
            .into_iter()
            .rev()
            .skip_while(|d| *d == 0)
            .map(|d| char::from_digit(d as u32, 16).unwrap())
            .collect()
    }
}

fn highest_fac(num: usize) -> usize {
    let mut acc = 0;
    while factorial(acc + 1) < num {
        acc += 1;
    }
    acc
}

fn factorial(num: usize) -> usize {
    match num {
        0 => 1,
        1 => 1,
        _ => num * factorial(num - 1),
    }
}
