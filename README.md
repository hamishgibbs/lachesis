# lachesis

## Description

Implements the visit detection algorithms proposed in Hariharan and Toyama (2004) *Project Lachesis: Parsing and Modeling Location Histories*

## Usage

``` {shell}
cat data/gps_data.csv | lachesis 200 300 > output.csv
```

## Limitations

This implementation is intentionally limited and relies only on the [Rust standard library](https://doc.rust-lang.org/std/) so that it can be installed without dependencies. 