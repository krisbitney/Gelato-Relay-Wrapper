use crate::wrap::imported::ArgsBufferKeccak256;
use crate::wrap::Sha3Module;

pub fn get_checksum_address(address: &str) -> String {
    let address_string = if address.starts_with("0x") {
        address[2..].to_lowercase()
    } else {
        address.to_lowercase()
    };

    let mut chars: Vec<char> = address_string.chars().collect();
    let mut expanded: Vec<u8> = Vec::new();
    for i in 0..40 {
        expanded.push(chars[i] as u8);
    }

    let hashed: String = Sha3Module::buffer_keccak_256(&ArgsBufferKeccak256 {
        message: expanded.clone()
    }).unwrap();
    let hashed_arr = arrayify(&hashed);

    let mut i = 0;
    while i < 40 {
        if hashed_arr[i >> 1] >> 4 >= 8 {
            chars[i] = chars[i].to_ascii_uppercase();
        }
        if (hashed_arr[i >> 1] & 0x0f) >= 8 {
            chars[i + 1] = chars[i + 1].to_ascii_uppercase();
        }
        i += 2;
    }

    "0x".to_string() + chars.into_iter().collect::<String>().as_str()
}

fn arrayify(hex: &str) -> Vec<u8> {
    if hex.starts_with("0x") {
        return arrayify(&hex[2..]);
    }
    let mut result: Vec<u8> = Vec::new();
    let mut i = 0;
    let mut j: usize = 0;
    while i < hex.len() {
        result[j] = u8::from_str_radix(&hex[i..i + 2], 16).unwrap();
        j += 1;
        i += 2;
    }
    result
}