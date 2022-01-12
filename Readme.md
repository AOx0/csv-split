# multi split-csv

Experimental branch

Simple command to split into n files the contents of one big csv.



## Installation

Simply run:

```
cargo install split-csv
```

Or use the GitHub version:
```
cargo install --git https://github.com/AOx0/csv-split
```

## Usage

```HELP
spcsv 0.0.9
Split a lage csv file into multiple files

error: could not compile `sinner` due to previous error
```



Example:

```
spcsv COVID19.csv 100
```



The example above will split the lines of `COVID19.csv` along a hundred files with the names: `COVID19_1.csv`, `COVID19_2.csv`, `COVID19_3.csv`, `...```
spcsv COVID19.csv 100



## Benchmarks

chronologer.yaml:

```yaml
benchmarks_dir: ./benchmarks/benchmarks
benchmark_runs: 100
benchmark_task: "{executable} /Users/alejandro/Downloads/Covid.csv 100"
build_command: "cargo build --release"
built_executable: ./target/release/spcsv
combined_benchmark_file: ./benchmarks/all-benchmarks.json
executables_dir: ./benchmarks/executables
html_output_file: ./benchmarks/index.html
revision_range: "320e28367ae45ee9bc3de2938e2b284d51238d2d..ee57d2915f16b8d36a21ba22957c5cbec3b78119"
```



Comparison of benchmarking results for `spcsv v0.0.8` and `multihread`'s `HEAD` splitting 12,133,531 rows from a csv of 1.8G of size into a 100 files:

![](https://raw.githubusercontent.com/AOx0/csv-split/multithread/misc/Test.png)



And the evolution of threads error range over the last 10 `multihread`'s commits:

![](https://raw.githubusercontent.com/AOx0/csv-split/multithread/misc/Test2.png)



Conclusion: Up to now, is better to stick with the stable non-multithread version `v0.0.8`
