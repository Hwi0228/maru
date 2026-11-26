use limine::{
    BaseRevision,
    request::{HhdmRequest, RequestsEndMarker, RequestsStartMarker},
};

// Base revision 0 temporarily retains the identity map and maps all memory-map
// regions through the HHDM. This lets the first AArch64 prototype reach the
// QEMU virt PL011 UART before MARU has its own MMIO mapper.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::with_revision(0);

#[used]
#[unsafe(link_section = ".requests")]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static REQUESTS_END: RequestsEndMarker = RequestsEndMarker::new();

pub fn base_revision_supported() -> bool {
    BASE_REVISION.is_supported()
}

pub fn hhdm_offset() -> u64 {
    // SAFETY: Limine owns the response memory for the duration of kernel execution.
    unsafe {
        HHDM_REQUEST
            .get_response()
            .expect("Limine did not provide the HHDM response")
            .offset()
    }
}
