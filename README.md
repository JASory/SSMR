# SSMR

WIP - Not all calculations are fully complete, will not be published to crates.io without them. This algorithm has been integrated and superceded by machine-prime

Single-Shot Miller-Rabin (SSMR) is an algorithm for primality testing using only a single strong fermat test to check if a number under a 
certain bound is prime.Like the related [Machine-prime](https://github.com/JASory/machine-prime) two functions are provided, is_prime and is_prime_wc. 
Is_prime is optimised for the average case and uses trial division in addition to the fermat test.Is_prime_wc is optimised for the worst case of checking primes, 
it may however pass even composites. This is because checking for divisibility by 2 is extremely fast,and by only constructing tests against 
odd composites we can double the interval we can construct the test. As well as the fact that most applications of is_prime_wc
will never encounter even composites.

Like Machine-prime it was also constructed using [F-Analysis](https://github.com/JASory/f-analysis), however it is likely not reproducible as the original 
algorithm has changed (nor does it need to be as it is much faster to exhaustively prove correctness, than to recalculate).

## Properties
 - Bound (is_prime) - 2^50 or 1.1 Quadrillion
 - is_prime_wc - Unknown probably errors less than 10K
 - is_prime Average Complexity - 0.21xFermat
 - is_prime_wc Worst-Case Complexity 1.0xFermat
 - Unlike most primality tests both functions have been exhaustively proven to behave exactly as documented, this is possible due to the small interval and their speed.
 - SSMR appears to be by far the largest interval for a single-shot miller-rabin test, the previous record was less than 1/256th the interval
 

## Applications

is_prime
- Testing other primality tests in the interval
- Determining primes of certain forms

is_prime_wc
- Testing sieves. Sieves generate mostly primes very rapidly, so using a test that verifies primes quickly is ideal
- Testing probable primes, like the results of partial factorisations 

## Possible Research Applications
- Searching for Carmichael numbers. Webster and Shallue calculated the Carmichael numbers up to 10^22, and since the
factors cannot be greater than sqrt(n),SSMR could be used to find Carmichael numbers up to 10^24. 
- Generating special semiprimes for constructing primality tests like numbers of the form (ak+1)(k+1)

## Shortcomings

Unfortunately SSMR has a current upper bound that precludes most applications in serious research. A specially designed sieve will outperform SSMR in virtually any application, the main barrier is difficulty in designing such a sieve.
Additionally a general purpose sieve like Kim Walisch's primesieve can already sieve up to 2^50 nearly instantaneously, rendering SSMR effectively useless at generating primes within some interval (Machine-primes purpose). 

This problem means that SSMR is currently relegated to recreational applications, however this is probably sufficient for most casual applications. One rarely needs to generate 900K primes per second, outside of research. 
