/*
#![no_std]
#![feature(lang_items)]
#![allow(internal_features)]
*/

//! Single-Shot Miller-Rabin (SSMR) is a specially designed primality test that uses only a single fermat test to check the primality 
//! of some integer less than 1099620565341 (approximately 1.09 trillion or 2^40). This means that it has the lowest computational complexity
//! known and is in practice the fastest published primality test for the provided interval. 
//!
//! Like [Machine-prime](https://github.com/JASory/machine-prime) SSMR was constructed using [F-Analysis](https://github.com/JASory/f-analysis)
pub(crate) mod check;
pub(crate) mod hashbase;
pub(crate) mod primes;

pub use check::is_prime;
pub use check::is_prime_wc;

/*
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

 End comment out*/
