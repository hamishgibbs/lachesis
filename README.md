# lachesis

## Description

Implements the visit detection algorithm from Hariharan and Toyama (2004) *Project Lachesis: Parsing and Modeling Location Histories*

## Data format

This library requires a `.csv` file (header optional) with the columns: `id` (String), `time` (String), `x` (Float), `y` (Float).

Observations must be ordered by id and time (ascending).

The visit detection algorithm requires parameters in the following order:
- The maximum spatial "roam" (i.e. 200m)
- The minium time duration of a visit (i.e. 300 seconds)
- A string speciying the [format](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) of dates in the input file (i.e. "%Y-%m-%d %H:%M:%S")

## Example usage

``` {shell}
cat data/gps_data.csv | lachesis 200 300 '%Y-%m-%d %H:%M:%S' > output.csv
```