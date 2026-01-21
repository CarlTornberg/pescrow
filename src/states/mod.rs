mod my_state;
use pinocchio::error::ProgramError;


/// Trait for structs which are of u8 byte arrays.
/// Used instead of repr(packed) for stability and predictability (?)
/// 
/// # SAFETY
/// - Struct must be constructed by u8 bytes
/// - All data must be initialized
pub unsafe trait Transmutable { 
    const LEN: usize;
}

/// Forms a slice from t of type T
#[inline(always)]
pub fn as_bytes<T: Transmutable>(t: &T) -> &[u8] {

    // SAFETY: T is of trait Transmutable
    unsafe {
        core::slice::from_raw_parts(
            t as *const T as *const u8, 
            size_of::<T>())
    }
}

#[inline(always)]
pub fn from_bytes<T: Transmutable>(data: &[u8]) -> Result<&T, ProgramError> {
    if data.len() != size_of::<T>() {
        return Err(ProgramError::BorshIoError); // TODO Change to a better error?
    }

    // SAFETY:
    // - data's length is the same as T
    // - Data is of trait Transmutable
    unsafe { Ok(from_bytes_unchecked(data)) }
}

/// Get data as a reference of type T
///
/// # SAFETY
/// - 'data' must be a valid representation of T
#[inline(always)]
pub unsafe fn from_bytes_unchecked<T: Transmutable>(data: &[u8]) -> &T {
    &*(data.as_ptr() as *const T)
}
