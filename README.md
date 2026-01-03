# hash-keccak-blake-cmp
vibe codded keccak, blake3, blake2s bench cmp



# Output of `cargo bench` on m4 cpu.

tldr:
* keccak256/32: 164.70 ns
* blake2s/32: 75.608 ns
* blake3/32: 50.233ns

Full output:

```rust
➜  hashcmp git:(master) ✗ cargo bench
    Finished `bench` profile [optimized] target(s) in 0.06s
     Running unittests src/main.rs (target/release/deps/hashcmp-082227469f052f83)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/hash_bench.rs (target/release/deps/hash_bench-c6cf155adbf86470)
Gnuplot not found, using plotters backend
keccak256/10            time:   [163.08 ns 163.60 ns 164.15 ns]
                        thrpt:  [58.099 MiB/s 58.294 MiB/s 58.478 MiB/s]
                 change:
                        time:   [−0.3464% +0.1805% +0.7097%] (p = 0.52 > 0.05)
                        thrpt:  [−0.7047% −0.1802% +0.3476%]
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
keccak256/32            time:   [164.41 ns 164.70 ns 165.02 ns]
                        thrpt:  [184.93 MiB/s 185.29 MiB/s 185.62 MiB/s]
                 change:
                        time:   [−5.1516% −4.1807% −3.3945%] (p = 0.00 < 0.05)
                        thrpt:  [+3.5138% +4.3632% +5.4314%]
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
keccak256/64            time:   [164.03 ns 164.65 ns 165.28 ns]
                        thrpt:  [369.28 MiB/s 370.69 MiB/s 372.10 MiB/s]
                 change:
                        time:   [−3.2921% −2.7262% −2.2113%] (p = 0.00 < 0.05)
                        thrpt:  [+2.2613% +2.8026% +3.4042%]
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) low mild
  7 (7.00%) high mild
  4 (4.00%) high severe
keccak256/100           time:   [168.17 ns 168.75 ns 169.39 ns]
                        thrpt:  [563.01 MiB/s 565.16 MiB/s 567.10 MiB/s]
                 change:
                        time:   [−0.4330% +0.0491% +0.5054%] (p = 0.85 > 0.05)
                        thrpt:  [−0.5028% −0.0491% +0.4349%]
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

blake2s/10              time:   [76.524 ns 77.103 ns 77.730 ns]
                        thrpt:  [122.69 MiB/s 123.69 MiB/s 124.62 MiB/s]
                 change:
                        time:   [−2.3708% −1.7605% −1.0965%] (p = 0.00 < 0.05)
                        thrpt:  [+1.1087% +1.7921% +2.4283%]
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe
blake2s/32              time:   [75.335 ns 75.608 ns 75.903 ns]
                        thrpt:  [402.06 MiB/s 403.63 MiB/s 405.09 MiB/s]
                 change:
                        time:   [−2.4759% −2.1332% −1.7551%] (p = 0.00 < 0.05)
                        thrpt:  [+1.7865% +2.1797% +2.5388%]
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  4 (4.00%) high severe
blake2s/64              time:   [77.384 ns 77.822 ns 78.275 ns]
                        thrpt:  [779.75 MiB/s 784.29 MiB/s 788.73 MiB/s]
                 change:
                        time:   [+0.9813% +1.4578% +1.9051%] (p = 0.00 < 0.05)
                        thrpt:  [−1.8694% −1.4369% −0.9717%]
                        Change within noise threshold.
blake2s/100             time:   [143.51 ns 143.79 ns 144.11 ns]
                        thrpt:  [661.77 MiB/s 663.23 MiB/s 664.55 MiB/s]
                 change:
                        time:   [−0.3801% −0.0159% +0.3268%] (p = 0.93 > 0.05)
                        thrpt:  [−0.3258% +0.0159% +0.3815%]
                        No change in performance detected.

blake3/10               time:   [50.481 ns 50.722 ns 50.962 ns]
                        thrpt:  [187.13 MiB/s 188.02 MiB/s 188.92 MiB/s]
                 change:
                        time:   [−3.3235% −2.9498% −2.6038%] (p = 0.00 < 0.05)
                        thrpt:  [+2.6734% +3.0395% +3.4377%]
                        Performance has improved.
blake3/32               time:   [50.011 ns 50.233 ns 50.472 ns]
                        thrpt:  [604.64 MiB/s 607.53 MiB/s 610.22 MiB/s]
                 change:
                        time:   [−2.5394% −1.2932% +0.3116%] (p = 0.08 > 0.05)
                        thrpt:  [−0.3107% +1.3102% +2.6055%]
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
blake3/64               time:   [49.312 ns 49.503 ns 49.699 ns]
                        thrpt:  [1.1993 GiB/s 1.2041 GiB/s 1.2087 GiB/s]
                 change:
                        time:   [−6.4013% −6.0400% −5.6754%] (p = 0.00 < 0.05)
                        thrpt:  [+6.0169% +6.4283% +6.8391%]
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
blake3/100              time:   [96.692 ns 96.843 ns 96.989 ns]
                        thrpt:  [983.28 MiB/s 984.76 MiB/s 986.30 MiB/s]
                 change:
                        time:   [−0.1537% +0.1440% +0.4244%] (p = 0.35 > 0.05)
                        thrpt:  [−0.4226% −0.1438% +0.1539%]
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  3 (3.00%) high mild
```