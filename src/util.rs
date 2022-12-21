use core::{convert::TryFrom as _, num::TryFromIntError};
pub(crate) fn overflow_error() -> TryFromIntError {
    u32::try_from(u64::MAX).unwrap_err()
}
