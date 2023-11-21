# RandCSV

A :sparkles: :fire: :zap: blazingly fast :zap: :fire: :sparkles: random csv generator written in rust with ChaCha12.

## Installation

Clone the repository, then run 
```sh
cargo build --release
```
Pre-built binaries might be available in the future.

## Usage

Running `--help` will give 
```
Usage: rand-csv [<columns...>] --output-file <output-file> --row-count <row-count>
```
where columns can be:
- `i`: 32 bit integer in the range 1-100
- `l`: double precision float in the range 1-100
- `s`: string with charset `[!?A-Za-z]`
- `d`: Date forwards from 1970
- `t`: Time
- `dt`: DateTime forwards from 1970

Custom ranges are allowed for integers, doubles, and string in the syntax `<type><start>:<end>`. For example, to generate an integer in the range -50 to 100 `i-50:100` would be used. For a string, the range is the string length.

## Example

The command
```sh
rand-csv i-50:100 l0:1 s10:10 d t dt --output-file foo.csv --row-count 5
```
Might generate the file `foo.csv` with the contents
```
93,0.042452890167961284,zqfqGIwvBur,31/05/2006,09:09,25/11/2053 09:43
69,0.6314582506910379,NUiIXdDrNvg,21/08/1983,18:40,05/11/1987 16:55
-24,0.5795932654730528,fMHzBNvwVKu,27/07/1977,00:25,08/06/2072 18:39
-10,0.8826483516999468,tB?zFHlIajn,15/09/2077,11:49,21/05/2085 04:14
-48,0.9154397366446112,KSMlBMsqpsb,31/07/1996,07:00,09/12/2014 06:40
```