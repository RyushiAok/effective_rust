// fuzz/fuzz_targets/fuzz_target_1.rs
#![no_main]
use libfuzzer_sys::fuzz_target;

pub fn is_fuzz(data: &[u8]) -> bool {
    if data.len() >= 3 && data[0] == b'F' && data[1] == b'U' && data[2] == b'Z' && data[3] == b'Z' {
        true
    } else {
        false
    }
}

fuzz_target!(|data: &[u8]| {
    let _ = is_fuzz(data);
});
