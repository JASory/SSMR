use crate::hashbase::FERMAT_BASE;
use crate::primes::{INV_8,PRIME_INV_64};
 
 fn mod_inv(n: u64) -> u64 {
        let mut est = INV_8[((n >> 1) & 0x7F) as usize] as u64;
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est
 }
 
 fn mont_sub(x: u64, y: u64, n: u64) -> u64 {
    if y > x {
        return n.wrapping_sub(y.wrapping_sub(x));
    }
    x.wrapping_sub(y)
}
 
  fn mont_prod(x: u64, y: u64, n: u64, npi: u64) -> u64 {
  
    let interim = x as u128 * y as u128;
    let lo = (interim as u64).wrapping_mul(npi);
    let lo = ((lo as u128 * n as u128)>>64) as u64;
    let hi = (interim>>64)  as u64;
    
    if hi < lo{
       hi - lo + n
    }
    else{
      hi-lo
    }
 }
 
 
 fn mont_pow(mut base: u64, mut one: u64, mut p: u64, n: u64, npi: u64) -> u64 {
    
  while p > 1 {
        if p & 1 == 0 {
            base = mont_prod(base, base, n, npi);
            p >>= 1;
        } else {
            one = mont_prod(one, base, n, npi);
            base = mont_prod(base, base, n, npi);
            p = (p - 1) >> 1;
        }
    }
    mont_prod(one, base, n, npi)
} 

fn sprp(p: u64, base: u64, one: u64, npi: u64) -> bool {
    let p_minus = p - 1;
    let twofactor = p_minus.trailing_zeros();
    let d = p_minus >> twofactor;
    
    let mut result = base.wrapping_mul(one);
    
    let oneinv = mont_prod(mont_sub(p,one,p),one,p,npi);
    
    result = mont_pow(result,one,d,p,npi);
    
    
    if result == one || result == oneinv {
        return true;
    }

    for _ in 1..twofactor {
        result = mont_prod(result, result, p, npi);

        if result == oneinv {
            return true;
        }
    }
    false
}

fn core_primality(x: u64) -> bool{
    let npi = mod_inv(x);
    let one = (u64::MAX % x) + 1; // 1359738876  //2202620065
    let idx = (x as u32).wrapping_mul(1359738876).wrapping_shr(18) as usize;
    sprp(x, FERMAT_BASE[idx] as u64, one,npi)

}

/// Fast primality in the worst case for all odd integers less than 1099620565341
pub fn is_prime_wc(x: u64) -> bool{
// SSMR passes some even composites due to the montgomery transform trick
  debug_assert!( (x != 0) && (x < 1099620565341) && ([1,4,14,16,18,90,418,1024,
                                     1248,1714,32208,48228,193152,
                                     456424,736232,749324,1659364,
                                     2037906,2157926,2512078,6510588,
                                     36070710,60809772,439998604,754749144,
                                     816051496,839707570,929747684,1964560406,
                                     2705505948,2722237320,3221225472,4382063290,
                                     7419460496,20524842874,28979528078].contains(&x) == false));
   core_primality(x)
}

// 1472,1892,1272,2998,173 {1248},27,3838,1573
//
/// Fast primality in the average case, for all integers less than 1099620565341
pub fn is_prime(x: u64) -> bool{

  debug_assert!(x < 1099620565341);
  
  if x == 1{
    return false
  }
  if x == 2{
    return true
  }
  
  if x&1==0{
    return false
  }
             for i in PRIME_INV_64.iter() {
                let prod = x.wrapping_mul(*i);
                if prod == 1 {
                    return true;
                }
                if prod < x {
                    return false;
                }
            }
            
            core_primality(x)

}
