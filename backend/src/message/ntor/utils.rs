use std::convert::TryInto;

/// Returns an array of zeros if conversion fails.
pub fn vec_to_array32(vec: Vec<u8>) -> [u8; 32] {
    if vec.len() == 32 {
        vec.try_into().unwrap()
    } else {
        [0u8; 32]
    }
}

