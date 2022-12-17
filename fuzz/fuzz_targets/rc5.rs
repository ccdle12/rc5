#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate rc5;
use rc5::rc5::Rc5_32_12_16;

fuzz_target!(|data: &[u8]| {
    let rc5_fuzzed_key = Rc5_32_12_16::new(&data[..]);

    let key = vec![0; 16];
    let rc5 = Rc5_32_12_16::new(&key[..]);

    if let Ok(r) = rc5 {
        r.encrypt(data.to_vec());
    }
});
