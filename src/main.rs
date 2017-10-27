
extern crate crypto;
extern crate rand;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::{thread_rng, Rng};
use std::fmt;
use std::time::Instant;
use std::env;

const DEFAULT_STEP : u64 = 1_000_000;

fn main() {
    let step = match env::args().nth(1) {
        Some(o) => o.parse::<u64>().unwrap_or(10u64) * DEFAULT_STEP ,
        None => DEFAULT_STEP,
    };
    println!("Printing every {}", step);
    let mut i : u64 = 0;
    let mut v = [0u8; 32];
    thread_rng().fill_bytes(&mut v);
    let start = Instant::now();
    let mut sha = Sha256::new();
    loop {
        if i%step == 0 {
            let n = (i / 1_000_000) as f64;
            let d = Instant::now().duration_since(start).as_secs() as f64;
            println!("{} {} {:6.3} Mhash/s",i ,HexSlice::new(&v), n / d );
        }
        sha.input(&v);
        sha.result(&mut v);
        sha.reset();
        i = i +1;
    }



}

struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    fn new<T>(data: &'a T) -> HexSlice<'a>
        where T: ?Sized + AsRef<[u8]> + 'a
    {
        HexSlice(data.as_ref())
    }
}

// You can even choose to implement multiple traits, like Lower and UpperHex
impl<'a> fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0 {
            // Decide if you want to pad out the value here
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crypto::digest::Digest;
    use crypto::sha2::Sha256;
    use ::HexSlice;

    #[test]
    pub fn test_hello_world_sha256() {
        let mut sha = Sha256::new();
        let input = "Hello world!".as_bytes();
        sha.input(input);
        assert_eq!(HexSlice::new(sha.result()), "c0535e4be2b79ffd93291305436bf889314e4a3faec05ecffcbb7df31ad9e51a");
    }
}