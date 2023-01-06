use once_cell::sync::Lazy;
use std::ffi::CStr;

type CpuCyclesFn = unsafe extern "C" fn() -> i64;

static CPU_CYCLES_FN: Lazy<CpuCyclesFn> = Lazy::new(|| {
    // First call finds best time source and updates the pointer
    unsafe { ffi::cpucycles.unwrap()() };
    unsafe { ffi::cpucycles.unwrap() }
});

pub struct CpuCycles(CpuCyclesFn);

impl CpuCycles {
    pub fn new() -> Self {
        CpuCycles(*CPU_CYCLES_FN)
    }

    pub fn version(&self) -> &'static str {
        unsafe {
            CStr::from_bytes_with_nul_unchecked(ffi::cpucycles_version)
                .to_str()
                .unwrap()
        }
    }

    pub fn cpucycles(&self) -> i64 {
        unsafe { self.0() }
    }

    pub fn cpucycles_implementation(&self) -> &'static str {
        unsafe {
            CStr::from_ptr(ffi::cpucycles_implementation())
                .to_str()
                .unwrap()
        }
    }

    pub fn cpucycles_persecond(&self) -> i64 {
        unsafe { ffi::cpucycles_persecond() }
    }
}

impl Default for CpuCycles {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_test() {
        assert_eq!("20230105", CpuCycles::new().version());
    }

    #[test]
    fn cpucycles_test() {
        let cpu_cycles = CpuCycles::new();
        let start = cpu_cycles.cpucycles();
        let end = cpu_cycles.cpucycles();
        assert!(end >= start);
    }

    #[test]
    fn cpucycles_persecond_test() {
        assert!(CpuCycles::new().cpucycles_persecond() > 0);
    }

    #[test]
    fn cpucycles_implementation_test() {
        // TODO: make this test pass on other arch/os configurations
        let expected = ["amd64-pmc", "amd64-tsc"];
        assert!(expected.contains(&CpuCycles::new().cpucycles_implementation()));
    }
}
