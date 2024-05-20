use rand::Rng;

use crate::opts::GenpassOpts;
use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(opts: &GenpassOpts) -> anyhow::Result<()> {
    // println!("genpass: {:?}", opts);
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();

    let mut chars = Vec::new();
    if opts.uppercase {
        chars.extend_from_slice(UPPER);
        password.push(UPPER[rng.gen_range(0..UPPER.len())] as u8);
    }
    if opts.lowercase {
        chars.extend_from_slice(LOWER);
        password.push(LOWER[rng.gen_range(0..LOWER.len())] as u8);
    }
    if opts.number {
        chars.extend_from_slice(NUMBER);
        password.push(NUMBER[rng.gen_range(0..NUMBER.len())] as u8);
    }
    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(SYMBOL[rng.gen_range(0..SYMBOL.len())] as u8);
    }

    for _ in 0..opts.length - password.len() as u8 {
        let idx = rng.gen_range(0..chars.len());
        password.push(chars[idx] as u8);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("{:?}", password);

    let estimate = zxcvbn(&password, &[])?;
    eprintln!("password strength: {}", estimate.score());

    Ok(())
}
