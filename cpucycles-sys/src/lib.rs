#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpucycles_test() {
        let start = unsafe { cpucycles.unwrap()() };
        let end = unsafe { cpucycles.unwrap()() };
        assert!(end >= start);
    }
}
