# lachesis

## Description

Implements the visit detection algorithm proposed in Hariharan and Toyama (2004) *Project Lachesis: Parsing and Modeling Location Histories*

## Data format

This library requires a `csv` file with the columns: `id` (integer), `time` (unix timestamp - integer), `x`, `y`.

## Usage

The visit detection algorithm requires the following parameters:
- The maximum spatial "roam" (200m below)
- The minium time duration of a visit (300 seconds below)

``` {shell}
cat data/gps_data.csv | lachesis 200 300 > output.csv
```

## Limitations

This implementation relies only on the [Rust standard library](https://doc.rust-lang.org/std/) so that it can be installed without dependencies. 