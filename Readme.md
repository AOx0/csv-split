# split-csv

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
error: D̴o̴n̸'̵t̷ ̷d̶e̵p̸l̶o̸y̵ ̵t̸h̷i̸s̸ ̵s̶h̴i̷t̷ ̵t̶o̴ ̷p̴r̸o̴d̶u̴c̷t̵i̵o̷n̴ ̷y̴o̷u̷ ̷m̶a̴d̸m̶a̸n̶
 --> /Users/alejandro/.cargo/registry/src/github.com-1ecc6299db9ec823/sinner-0.1.2/src/lib.rs:5:1
  |
5 | compile_error!("D̴o̴n̸'̵t̷ ̷d̶e̵p̸l̶o̸y̵ ̵t̸h̷i̸s̸ ̵s̶h̴i̷t̷ ̵t... ̷p̴r̸o̴d̶u̴c̷t̵i̵o̷n̴ ̷y̴o̷u̷ ̷m̶a̴d̸m̶a̸n̶");
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: could not compile `sinner` due to previous error
```



Example:

```
spcsv COVID19.csv 100
```



The example above will split the lines of `COVID19.csv` along a hundred files with the names: `COVID19_1.csv`, `COVID19_2.csv`, `COVID19_3.csv`, `...```
spcsv COVID19.csv 100
```



The example above will split the lines of `COVID19.csv` along a hundred files with the names: `COVID19_1.csv`, `COVID19_2.csv`, `COVID19_3.csv`, `...`