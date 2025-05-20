// fuzz/fuzz_targets/fuzz_target_1.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use effective_rust::parse_message;

fuzz_target!(|data: &[u8]| {
    let _ = parse_message(data);
});
