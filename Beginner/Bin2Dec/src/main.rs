use anyhow::{Result, bail};
use std::{convert::TryFrom, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("invalid input")]
    InvalidInput,
    #[error("overflow")]
    Overflow,
}

fn bin2dec(input: String) -> Result<String> {
    let mut dec: i32 = 0;
    for (i, c) in input.into_bytes().into_iter().rev().enumerate() {
        let bit = match c {
            b'0' => 0i32,
            b'1' => 1i32,
            _ => bail!(CustomError::InvalidInput),
        };

        // dec += c * 2 ^ i
        let exp = u32::try_from(i)?;
        let pow = 2i32.checked_pow(exp).ok_or(CustomError::Overflow)?;
        let mul = bit.checked_mul(pow).ok_or(CustomError::Overflow)?;
        dec = dec.checked_add(mul).ok_or(CustomError::Overflow)?;
    }

    Ok(dec.to_string())
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    loop {
        let mut buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(_) => {
                buffer = buffer.trim().to_string();
                match bin2dec(buffer) {
                    Ok(dec) => {
                        println!("decimal: {}", dec);
                    }
                    Err(error) => {
                        println!("error: {}", error);
                    }
                };
            }
            Err(error) => {
                println!("error: {:?}", error);
            }
        };
    }
}
