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



The example above will split the lines of `COVID19.csv` along a hundred files names `COVID19-1.csv`, `COVID19-2.csv`, `...`



