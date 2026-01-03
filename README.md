# hash-keccak-blake-cmp
vibe codded keccak, blake3, blake2s bench cmp



# Output of cargo bench

```bash
     Running benches/hash_bench.rs (target/release/deps/hash_bench-3554138289444320)
Gnuplot not found, using plotters backend
keccak256/10            time:   [164.09 ns 164.36 ns 164.64 ns]
                        thrpt:  [57.924 MiB/s 58.024 MiB/s 58.120 MiB/s]
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) low severe
  5 (5.00%) low mild
  8 (8.00%) high mild
keccak256/32            time:   [169.60 ns 170.55 ns 171.68 ns]
                        thrpt:  [177.76 MiB/s 178.93 MiB/s 179.93 MiB/s]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
keccak256/64            time:   [168.32 ns 169.14 ns 170.02 ns]
                        thrpt:  [358.99 MiB/s 360.86 MiB/s 362.61 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) low mild
  7 (7.00%) high mild
  3 (3.00%) high severe
keccak256/100           time:   [168.90 ns 169.15 ns 169.40 ns]
                        thrpt:  [562.97 MiB/s 563.82 MiB/s 564.63 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) high mild
  6 (6.00%) high severe

blake2s/10              time:   [77.319 ns 77.660 ns 78.094 ns]
                        thrpt:  [122.12 MiB/s 122.80 MiB/s 123.34 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) low severe
  3 (3.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe
blake2s/32              time:   [76.423 ns 76.594 ns 76.773 ns]
                        thrpt:  [397.50 MiB/s 398.43 MiB/s 399.33 MiB/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild
blake2s/64              time:   [76.258 ns 76.454 ns 76.695 ns]
                        thrpt:  [795.82 MiB/s 798.33 MiB/s 800.37 MiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
blake2s/100             time:   [144.03 ns 144.72 ns 145.47 ns]
                        thrpt:  [655.60 MiB/s 658.96 MiB/s 662.14 MiB/s]
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low mild
  8 (8.00%) high mild
  4 (4.00%) high severe

blake3/10               time:   [52.135 ns 52.208 ns 52.285 ns]
                        thrpt:  [182.40 MiB/s 182.67 MiB/s 182.92 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) low mild
  2 (2.00%) high mild
blake3/32               time:   [51.816 ns 51.912 ns 52.013 ns]
                        thrpt:  [586.73 MiB/s 587.87 MiB/s 588.96 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high severe
blake3/64               time:   [52.290 ns 52.373 ns 52.455 ns]
                        thrpt:  [1.1363 GiB/s 1.1381 GiB/s 1.1399 GiB/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild
blake3/100              time:   [95.895 ns 96.130 ns 96.401 ns]
                        thrpt:  [989.28 MiB/s 992.07 MiB/s 994.50 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
```