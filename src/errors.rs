use pinocchio::error::ProgramError;

#[repr(u8)]
pub enum CustomError {

    TransmutableError,

    UnknownError = u8::MAX,
}

impl From<u8> for CustomError {
    fn from(value: u8) -> Self {
        match value {
            0..=0 => {
                unsafe { core::mem::transmute::<u8, CustomError>(value) }
            }
            _ => CustomError::UnknownError
        }
    }
}

impl From<CustomError> for u8 {
    fn from(value: CustomError) -> Self {
        value as Self        
    }
}


impl From<CustomError> for ProgramError {
    fn from(value: CustomError) -> Self {
        ProgramError::Custom(value as u32)
        
    }
}

