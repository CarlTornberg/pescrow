use pinocchio::error::ProgramError;

#[repr(u8)]
pub enum CustomError {

    TransmutableError,

    UnknownError = u8::MAX,
}

impl From<u8> for CustomError {
    fn from(value: u8) -> Self {
        match value {
            0..=1 => {
                unsafe { core::mem::transmute::<u8, CustomError>(value) }
            }
            _ => CustomError::UnknownError
        }
    }
}

impl Into<u8> for CustomError {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<ProgramError> for CustomError {
    fn into(self) -> ProgramError {
        ProgramError::Custom(self as u32)
    }
}
