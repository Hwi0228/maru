#![no_std]

/// Convert a physical address into its Higher-Half Direct Map virtual address.
///
/// Returns None if the addition overflows or the resulting address cannot
/// be represented by the current target's pointer width.
pub fn hhdm_address(hhdm_offset: u64, physical_address: u64) -> Option<usize> {
    let virtual_address = hhdm_offset.checked_add(physical_address)?;
    usize::try_from(virtual_address).ok()
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::hhdm_address;

    #[test]
    fn zero_offset_preserves_physical_address() {
        assert_eq!(hhdm_address(0, 0x0900_0000), Some(0x0900_0000));
    }

    #[test]
    fn adds_hhdm_offset_to_physical_address() {
        assert_eq!(
            hhdm_address(0xffff_8000_0000_0000, 0x0900_0000),
            Some(0xffff_8000_0900_0000usize)
        );
    }

    #[test]
    fn rejects_u64_addition_overflow() {
        assert_eq!(hhdm_address(u64::MAX, 1), None);
    }

    #[test]
    fn accepts_zero_address() {
        assert_eq!(hhdm_address(0, 0), Some(0));
    }
}
