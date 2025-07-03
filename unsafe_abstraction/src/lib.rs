#![cfg_attr(not(test), no_std)]

use core::mem::MaybeUninit;

/// This is definitely VERY bad code, but it is used as an example for Miri to catch an obvious error.
pub unsafe fn random_i32() -> i32 {
    let result = MaybeUninit::uninit();
    result.assume_init()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_i32() {
        // This emits a runtime error if called by Miri
        let value = unsafe { random_i32() };
        println!("Random value: {value}");
    }
}
