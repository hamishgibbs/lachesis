# lachesis

## Description

Implements the first-stage stop point detection algorithm from Hariharan and Toyama (2004) *Project Lachesis: Parsing and Modeling Location Histories*

## Data format

This library requires a `.csv` file (header optional) with the columns: `id` (String), `time` (String), `x` (Float), `y` (Float).

## Example usage

``` {shell}
cat data/gps_data.csv | lachesis --distance 200 --time 300 --fmt-time '%Y-%m-%d %H:%M:%S' > output.csv
```

## Help

```
lachesis --help
```

```
GPS stop point detection from Hariharan and Toyama (2004) 'Project Lachesis: Parsing and Modeling Location Histories'

Usage: lachesis --distance <DISTANCE> --time <TIME> --fmt-time <FMT_TIME>

Options:
  -d, --distance <DISTANCE>  Maximum spatial roam of a stop point (i.e. 200m)
  -t, --time <TIME>          Minimum time duration of a stop point (i.e. 300 seconds)
  -f, --fmt-time <FMT_TIME>  Format of dates in the input file (i.e. "%Y-%m-%d %H:%M:%S")
  -h, --help                 Print help
  -V, --version              Print version
```
