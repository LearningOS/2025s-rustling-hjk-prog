/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
/// The caller must ensure that the address is valid and points to a mutable `u32`.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: The caller guarantees that the address is valid and points to a mutable `u32`.
    // Therefore, converting the address to a raw pointer and dereferencing it is safe.
    unsafe {
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert_eq!(t, 0xAABBCCDD);
    }
}