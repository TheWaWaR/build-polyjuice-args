use clap::{App, Arg};
use numext_fixed_hash::{H160, H256};
use numext_fixed_uint::U256;

fn main() {
    let matches = App::new("polyjuice evm message builder")
        .arg(
            Arg::with_name("depth")
                .long("depth")
                .short("d")
                .takes_value(true)
                .default_value("0")
                .help("The call depth"),
        )
        .arg(
            Arg::with_name("tx-origin")
                .long("tx-origin")
                .takes_value(true)
                .default_value("0x0000000000000000000000000000000000000000")
                .help("The transaction original sender"),
        )
        .arg(
            Arg::with_name("call-kind")
                .long("call-kind")
                .short("k")
                .takes_value(true)
                .default_value("call")
                .possible_values(&["call", "delegatecall", "callcode", "create", "create2"])
                .help("The call kind"),
        )
        .arg(
            Arg::with_name("static-call")
                .long("static-call")
                .help("The flag for EVM message"),
        )
        .arg(
            Arg::with_name("value")
                .long("value")
                .short("v")
                .takes_value(true)
                .default_value("0x0000000000000000000000000000000000000000000000000000000000000000")
                .help("The amount of Ether transferred with the message."),
        )
        .arg(
            Arg::with_name("input-data")
                .long("input-data")
                .takes_value(true)
                .required(true)
                .default_value("0x")
                .help("The input data for EVM message."),
        )
        .get_matches();

    let depth: u16 = matches.value_of("depth").unwrap().parse().unwrap();
    let tx_origin_opt: Option<H160> = if depth > 0 {
        Some(H160::from_hex_str(remove_hex_prefix(matches.value_of("tx-origin").unwrap())).unwrap())
    } else {
        None
    };
    let kind: u8 = match matches.value_of("call-kind").unwrap() {
        "call" => 0,
        "delegatecall" => 1,
        "callcode" => 2,
        "create" => 3,
        "create2" => 4,
        _ => unreachable!(),
    };
    let flags: u8 = if matches.is_present("static-call") {
        1
    } else {
        0
    };
    let value_data: H256 =
        H256::from_hex_str(remove_hex_prefix(matches.value_of("value").unwrap())).unwrap();
    let value: U256 = U256::from_be_bytes(value_data.as_fixed_bytes());
    let input_str = matches.value_of("input-data").unwrap();
    let input_data: Vec<u8> = hex::decode(remove_hex_prefix(input_str)).unwrap();

    println!(
        "0x{}",
        make_bytes(
            depth,
            tx_origin_opt.as_ref(),
            kind,
            flags,
            &value,
            &input_data[..]
        )
    );
}

fn make_bytes(
    depth: u16,
    tx_origin_opt: Option<&H160>,
    kind: u8,
    flags: u8,
    value: &U256,
    input_data: &[u8],
) -> String {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend(&depth.to_le_bytes());
    if let Some(tx_origin) = tx_origin_opt {
        buf.extend(tx_origin.as_bytes());
    }
    buf.push(kind);
    buf.push(flags);
    buf.extend(&value.to_le_bytes());
    buf.extend(&(input_data.len() as u32).to_le_bytes());
    buf.extend(input_data);
    hex::encode(&buf)
}

fn remove_hex_prefix(mut input_str: &str) -> &str {
    if &input_str[0..2] == "0x" || &input_str[0..2] == "0X" {
        input_str = &input_str[2..];
    }
    input_str
}

#[cfg(test)]
mod test {

    use super::*;
    use numext_fixed_hash::h160;
    use numext_fixed_uint::u256;
    #[test]
    fn test_make_bytes() {
        assert_eq!(
            make_bytes(0, None, 3, 0, &u256!("0x0"), &[][..]),
            "00000300000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert_eq!(
            make_bytes(1, Some(&h160!("0x3300000000000000000000000000000000003333")), 0, 1, &u256!("0x0"), &[0x33][..]),
            "01003300000000000000000000000000000000003333000100000000000000000000000000000000000000000000000000000000000000000100000033",
        );
    }
}
