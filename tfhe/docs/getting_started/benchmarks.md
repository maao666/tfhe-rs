# Benchmarks

Due to their nature, homomorphic operations are naturally slower than their clear equivalent. Some timings are exposed for basic operations. For completeness, benchmarks for other libraries are also given.

All benchmarks were launched on an AWS m6i.metal with the following specifications: Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz and 512GB of RAM.

## Boolean

This measures the execution time of a single binary Boolean gate.

### tfhe-rs::boolean.

| Parameter set         | Concrete FFT | Concrete FFT + avx512 |
| --------------------- | ------------ | --------------------- |
| DEFAULT\_PARAMETERS   | 8.8ms        | 6.8ms                 |
| TFHE\_LIB\_PARAMETERS | 13.6ms       | 10.9ms                |

### tfhe-lib.

| Parameter set                                    | fftw   | spqlios-fma |
| ------------------------------------------------ | ------ | ----------- |
| default\_128bit\_gate\_bootstrapping\_parameters | 28.9ms | 15.7ms      |

### OpenFHE.

| Parameter set | GINX  | GINX (Intel HEXL) |
| ------------- | ----- | ----------------- |
| STD\_128      | 172ms | 78ms              |
| MEDIUM        | 113ms | 50.2ms            |


## Shortint
This measures the execution time for some operations and some parameter sets of tfhe-rs::shortint.

This uses the Concrete FFT + avx512 configuration.

| Parameter set               | unchecked\_add | unchecked\_mul\_lsb | keyswitch\_programmable\_bootstrap |
| --------------------------- | -------------- | ------------------- | ---------------------------------- |
| PARAM\_MESSAGE\_1\_CARRY\_1 | 338 ns         | 8.3 ms              | 8.1 ms                             |
| PARAM\_MESSAGE\_2\_CARRY\_2 | 406 ns         | 18.4 ms             | 18.4 ms                            |
| PARAM\_MESSAGE\_3\_CARRY\_3 | 3.06 µs        | 134 ms              | 129.4 ms                           |
| PARAM\_MESSAGE\_4\_CARRY\_4 | 11.7 µs        | 854 ms              | 828.1 ms                           |

Next, the timings for the operation flavor `default` are given. This flavor ensures predictable timings of an operation all along the circuit by clearing the carry space after each operation.

| Parameter set               |            add |        mul\_lsb     | keyswitch\_programmable\_bootstrap |
| --------------------------- | -------------- | ------------------- | ---------------------------------- |
| PARAM\_MESSAGE\_1\_CARRY\_1 | 7.90 ms        | 8.00 ms             | 8.10 ms                            |
| PARAM\_MESSAGE\_2\_CARRY\_2 | 18.4 ms        | 18.1 ms             | 18.4 ms                            |
| PARAM\_MESSAGE\_3\_CARRY\_3 | 131.5 ms       | 129.5 ms            | 129.4 ms                           |
| PARAM\_MESSAGE\_4\_CARRY\_4 | 852.5 ms       | 839.7 ms            | 828.1 ms                           |


## Integer
This measures the execution time for some operation sets of tfhe-rs::integer.

All timings are related to parallelized Radix-based integer operations, where each block is encrypted using PARAM\_MESSAGE\_2\_CARRY\_2.
To ensure predictable timings, the operation flavor is the `default` one: a carry propagation is computed after each operation. Operation cost could be reduced by using `unchecked`, `checked`, or `smart`.

| Plaintext size     |  add           | mul                 | greater\_than (gt)   |  min         |
| -------------------| -------------- | ------------------- | ---------            | -------      |
| 8    bits          | 129.0 ms       | 178.2 ms            | 111.9 ms             | 287.7 ms     |
| 16   bits          | 256.3 ms       | 328.0 ms            | 145.3 ms             |  437.4 ms    |
| 32   bits          | 469.4 ms       | 645.5 ms            | 192.0 ms             |  776.4 ms    |
| 40   bits          | 608.0 ms       | 849.3 ms            | 228.4 ms             |  953.5 ms    |
| 64   bits          | 959.9 ms       | 1.49 s              | 249.0 ms             |  1.36 s      |
| 128  bits          | 1.88 s         | 3.25 s              | 294.7 ms             |  2.37 s      |
| 256  bits          | 3.66 s         | 8.38 s              | 361.8 ms             |  4.51 s      |
