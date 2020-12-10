
# Build godwoken polyjuice layer 2 contract args


## Build generator arguments

A [usage example](https://github.com/TheWaWaR/godwoken/blob/7d9f0e835c93def4f1ade4ebdaa27683411da406/crates/generator/src/tests/examples.rs#L22-L79) in godwoken.
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
    -k, --call-kind <call-kind>      The call kind [default: call]  [possible values: call, delegatecall, callcode,
                                     create, create2]
    -d, --depth <depth>              The call depth [default: 0]
        --input-data <input-data>    The input data for EVM message. [default: 0x]
        --tx-origin <tx-origin>      The transaction original sender [default:
                                     0x0000000000000000000000000000000000000000]
    -v, --value <value>              The amount of Ether transferred with the message. [default:
                                     0x0000000000000000000000000000000000000000000000000000000000000000]
```

Frame structure (`depth = 0`)
```
args[0..2]               => depth
args[2..3]               => call_kind
args[3..4]               => flags
args[4..36]              => value
args[36..40]             => input_size
args[40..40+input_size]  => input_data
```

Frame structure (`depth > 0`)
```
args[0..2]               => depth
args[2..22]              => tx_origin
args[22..23]             => call_kind
args[23..24]             => flags
args[24..56]             => value
args[56..60]             => input_size
args[60..60+input_size]  => input_data
```
