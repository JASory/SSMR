// Benchmarking is performed on sequential integers
// Throughput is approximately 28 million integers per second
use ssmr::{is_prime,is_prime_wc};

// Benchmarking the average in the strongest interval
// Real implementation will likely be faster  
fn bench_average(){

   const DELTA : u64 = 100_000_000;
   const INF : u64 = 1099620565341-DELTA;
   const SUP : u64 = INF + DELTA;
   let start = std::time::Instant::now();
   let mut count = 0;
   for i in INF..SUP{
      if is_prime(i){
        count+=1;
      }
   }
   let stop = start.elapsed();
   println!("Finished in t: {:?}", stop);
   println!("{} integers evaluated per second, finding {} primes",(DELTA/stop.as_millis() as u64)*1000, count);
}

// Benchmarking against the strongest case, real world will be faster
// Throughput is approximately 4.1 million integers per second
fn bench_worst(){
  const ITERATIONS : u64 = 10_000_000;
   
  const VALUE : u64 = 1099620565321;
  
   let start = std::time::Instant::now();
   let mut count : u64 = 0;
   for _ in 0..ITERATIONS{
      if is_prime_wc(VALUE){
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
