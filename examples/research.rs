use ssmr::{is_prime,is_prime_wc};


// Counts the semiprimes that satisfy the Monier-Rabin bound, they are of the form (2x+1)(4x+1)
// A realistic application would initialise the lhs with a sieve and use SSMR for the much rarer rhs 
// In practice SSMR is faster if we are dealing with sparse semiprime components that are larger than the initial factor. 
fn mr_count() -> u64{
  let mut count = 0u64;
   for lhs in 0..1u64<<32{
     if is_prime(lhs){
        // Construct the candidate integer
        let rhs = (((lhs-1)>>2)<<1)+1;
        // Check if candidate integer is also prime
        if is_prime_wc(rhs){
        count+=1;
        }
     }
   }
   count
}

 
fn main(){

  let start = std::time::Instant::now();
  let count  = mr_count();
  let stop = start.elapsed();
  
  println!("{} {:?}",count,stop)
}
