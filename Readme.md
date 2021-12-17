# split-csv

Simple command to split into n files the contents of one big csv.



## Installation

Simply run:

```
cargo install split-csv
```



## Usage

```
spcsv [Options] <file> <number_files>
```



Example:

```
spcsv COVID19.csv 100
```



The example above will split the lines of `COVID19.csv` along a hundred files with the names: `COVID19_1.csv`, `COVID19_2.csv`, `COVID19_3.csv`, `...`



