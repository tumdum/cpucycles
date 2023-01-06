use once_cell::sync::Lazy;
use std::ffi::CStr;

type CpuCyclesFn = unsafe extern "C" fn() -> i64;

static CPU_CYCLES_FN: Lazy<CpuCyclesFn> = Lazy::new(|| {
    // First call finds best time source and updates the pointer
    unsafe { ffi::cpucycles.unwrap()() };
    unsafe { ffi::cpucycles.unwrap() }
});

#[derive(Clone, Copy)]
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

    pub fn get(&self) -> i64 {
        unsafe { self.0() }
    }

    pub fn implementation(&self) -> &'static str {
        unsafe {
            CStr::from_ptr(ffi::cpucycles_implementation())
                .to_str()
                .unwrap()
        }
    }

    pub fn per_second(&self) -> i64 {
        unsafe { ffi::cpucycles_persecond() }
    }

    pub fn instant(&self) -> Instant {
        Instant {
            start: self.get(),
            cpu_cycles: *self,
        }
    }
}

impl Default for CpuCycles {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
pub struct Instant {
    start: i64,
    cpu_cycles: CpuCycles,
}

impl Instant {
    pub fn now() -> Self {
        let cpu_cycles = CpuCycles::new();
        let start = cpu_cycles.get();
        Instant { start, cpu_cycles }
    }

    pub fn elapsed(&self) -> i64 {
        self.cpu_cycles.get() - self.start
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
        let start = cpu_cycles.get();
        let end = cpu_cycles.get();
        assert!(end >= start);
    }

    #[test]
    fn cpucycles_persecond_test() {
        assert!(CpuCycles::new().per_second() > 0);
    }

    #[test]
    fn cpucycles_implementation_test() {
        // TODO: make this test pass on other arch/os configurations
        let expected = ["amd64-pmc", "amd64-tsc", "amd64-tscasm"];
        assert!(expected.contains(&CpuCycles::new().implementation()));
    }

    #[test]
    fn instant_test() {
        let instant = Instant::now();
        assert!(instant.elapsed() >= 0);

        let instant = CpuCycles::new().instant();
        assert!(instant.elapsed() >= 0);
    }
}
