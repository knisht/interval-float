use std::os::raw::c_int;

pub use float::Float;

pub use crate::interval::Interval;

mod interval;
mod float;

#[link(name = "fcontrols", kind = "static")]
extern "C" {
    fn round_upward() -> ();
    fn round_restore() -> ();
    fn get_state() -> c_int;
}

pub fn interval_computation<T, F: FnOnce() -> T>(closure: F) -> T {
    unsafe { round_upward() };
    let result = closure();
    // todo: reentrancy issues and exception handling
    unsafe { round_restore() };
    result
}


#[cfg(test)]
mod tests {
    use crate::interval::Interval;
    use crate::interval_computation;

    #[test]
    fn creation() {
        interval_computation(|| {
            let interval = Interval::from(0.1337);
            assert_eq!(interval.high - interval.low, 0.)
        })
    }

    #[test]
    fn addition() {
        interval_computation(|| {
            let sum = Interval::from(0.1) + Interval::from(0.2);
            assert_ne!(sum.high - sum.low, 0.);
        })
    }

    #[test]
    #[should_panic]
    fn usage_without_initialized_context() {
        Interval::<f32>::new();
    }
}
