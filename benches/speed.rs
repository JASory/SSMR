// Benchmarking is performed on sequential integers 
// Enumerating the entire interval of 0;2^37 takes 4812 seconds with a throughput of 28,561,711 integers per second
// Checking the strongest prime has a throughput of 4,196,391
use ssmr::{is_prime,is_prime_wc};

// Benchmarking the average in the strongest interval
// Real implementation will likely be faster  
fn bench_average(){

   const DELTA : u64 = 100_000_000;
   const SUP : u64 = 1<<37;
   const INF : u64 = (1<<37) - DELTA;
   let start = std::time::Instant::now();
   let mut count = 0;
   for i in INF..SUP{
      if is_prime(i){
        count+=1
      }
   }
   let stop = start.elapsed();
   println!("Finished in t: {:?}", stop);
   println!("{} integers evaluated per second, finding {} primes",(DELTA/stop.as_millis() as u64)*1000, count);
}

// Bencharking against the strongest case, real world will be faster
fn bench_worst(){
  const ITERATIONS : u64 = 10_000_000;
   
   let start = std::time::Instant::now();
   let mut count : u64 = 0;
   for _ in 0..ITERATIONS{
      if is_prime_wc(137438953447){
        count+=1
      }
   }
   let stop = start.elapsed();
   println!("Finished in t: {:?}", stop);
   println!("{} integers evaluated per second",(ITERATIONS/stop.as_millis() as u64)*1000);
   assert_eq!(count,ITERATIONS)
}

fn main(){
  bench_average();
  bench_worst();
}
