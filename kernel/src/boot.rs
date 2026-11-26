use limine::{
    BaseRevision,
    request::{HhdmRequest, RequestsEndMarker, RequestsStartMarker},
};

// Limine's current AArch64 protocol requires base revision 6 or newer.
// x86_64 retains revision 0 for the prototype's legacy identity-map setup.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();

#[cfg(target_arch = "x86_64")]
#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::with_revision(0);

#[cfg(target_arch = "aarch64")]
#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::with_revision(6);

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
    HHDM_REQUEST
        .get_response()
        .expect("Limine did not provide the HHDM response")
        .offset()
}
