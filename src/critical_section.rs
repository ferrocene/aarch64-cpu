#[cfg(feature = "critical-section-single-core")]
mod single_core {
    use core::sync::atomic;

    use crate::registers::{self, Readable, Writeable};

    struct SingleCoreCriticalSection;

    critical_section::set_impl!(SingleCoreCriticalSection);

    const DAIF_OFFSET: usize = 6;

    unsafe impl critical_section::Impl for SingleCoreCriticalSection {
        unsafe fn acquire() -> critical_section::RawRestoreState {
            let daif_bits = registers::DAIF.get();

            registers::DAIF.write(
                registers::DAIF::D::Masked
                    + registers::DAIF::A::Masked
                    + registers::DAIF::I::Masked
                    + registers::DAIF::F::Masked,
            );

            // prevent reordering across the preceding register write
            atomic::compiler_fence(atomic::Ordering::SeqCst);

            // shift DAIF value (only 4 contiguous bits are used) to fit into `u8`
            (daif_bits >> DAIF_OFFSET) as u8
        }

        unsafe fn release(daif_bits: critical_section::RawRestoreState) {
            // prevent reordering across the following register write
            atomic::compiler_fence(atomic::Ordering::SeqCst);

            registers::DAIF.set((daif_bits << DAIF_OFFSET) as u64);
        }
    }
}
