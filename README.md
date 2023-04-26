# lachesis

## Description

Implements the visit detection algorithm from Hariharan and Toyama (2004) *Project Lachesis: Parsing and Modeling Location Histories*

## Data format

This library requires a `.csv` file (no headers) with the columns: `id` (String), `time` (unix timestamp - Integer), `x` (Float), `y` (Float).

Observations must be ordered by id and time (ascending).

The visit detection algorithm requires parameters in the following order:
- The maximum spatial "roam" (i.e. 200m)
- The minium time duration of a visit (i.e. 300 seconds)

## Example usage

``` {shell}
cat data/gps_data.csv | lachesis 200 300 > output.csv
```