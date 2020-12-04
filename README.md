
# The tool to build godwoken polyjuice layer 2 contract args


## Build generator arguments
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
