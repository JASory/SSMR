# SSMR
Single-Shot Miller Rabin is the fastest primality test for small integers


Most casual recreational mathematics applications of primality only deal with small integers. This primality test tries to be the fastest possible for those integers. It is currently accurate for values slightly greater than 2^37 (137512792873). This is approximately 32 times higher than any other primality test that uses a single base. 

SSMR is approximately 25% faster than Machine-Prime for the is_prime function and 45% faster for the is_prime_wc function. Like Machine-prime it is constructed by the f-analysis library. 
