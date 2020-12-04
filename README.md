
# The tool to build godwoken polyjuice layer 2 contract args


## Build generator arguments

A [usage example](https://github.com/TheWaWaR/godwoken/blob/7d9f0e835c93def4f1ade4ebdaa27683411da406/crates/generator/src/tests/examples.rs#L22-L79)
```
$ cargo run -- --help
polyjuice evm message builder 

USAGE:
    build-polyjuice-args [FLAGS] [OPTIONS] --input-data <input-data>

FLAGS:
    -h, --help           Prints help information
        --static-call    The flag for EVM message
    -V, --version        Prints version information

OPTIONS:
        --depth <depth>              The call depth [default: 0]
        --input-data <input-data>    The input data for EVM message. [default: 0x]
        --tx-origin <tx-origin>      The transaction original sender [default:
                                     0x0000000000000000000000000000000000000000]
        --value <value>              The amount of Ether transferred with the message. [default:
                                     0x0000000000000000000000000000000000000000000000000000000000000000]
```

Frame structure (depth = 0)
```
args[0..2]               => depth
args[2..3]               => flags
args[3..35]              => value
args[35..39]             => input_size
args[39..39+input_size]  => input_data
```

Frame structure (depth > 0)
```
args[0..2]               => depth
args[2..22]              => tx_origin
args[22..23]             => flags
args[23..55]             => value
args[55..59]             => input_size
args[59..59+input_size]  => input_data
```
