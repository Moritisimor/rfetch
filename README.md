# RFetch
CLI HTTP Client written in Rust.

## What is this project about?
RFetch is a simple but ergonomic CLI-Based HTTP Client. Functionally it is similar to curl but with more understandable flag arguments.

## Example
```bash
rfetch -u http://localhost:8080/hello -j -b '{"content": "Hello Server!"}' -m POST --head mycustomheader:myvalue
```

### Flags:
| ```-u``` | ```-j``` | ```-b``` | ```-m``` | ```--head``` |
| -------- | -------- | -------- | -------- | ------------ |
|   URL    |   JSON   |   Body   |  Method  |    Header    |

## Compilation
First, clone this repository
```bash
git clone https://github.com/Moritisimor/rfetch
```

Then, cd into the source code folder
```bash
cd rfetch/src
```

And finally compile it with cargo
```bash
cargo build -r
```

The binary will be in ```rfetch/target/release``` and is called ```rfetch```
