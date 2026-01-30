use core::{mem::MaybeUninit, ptr::copy_nonoverlapping};
use pinocchio::error::ProgramError;
use crate::{errors::CustomError, helpers::bytes_helpers::Transmutable, types::Discriminator};

#[repr(u8)]
pub enum ProgramInstructions {
    Make = 0,
    Take = 1,
    Refund = 2,
}

/// SAFETY Is a single byte in size
unsafe impl Transmutable for ProgramInstructions { 
    const LEN: usize = size_of::<Self>();
}

impl From<ProgramInstructions> for Discriminator {
    fn from(value: ProgramInstructions) -> Self {
        value as Discriminator
    }
}

/// Not the most efficient solution? transmute copies the src to dst
impl TryFrom<u8> for ProgramInstructions {
    type Error = ProgramError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value { 
            // SAFETY: 
            // - Keep the range up to date when adding new instructions.
            // - ProgramInstruction is represented as a u8
            0..=3 => Ok(unsafe { core::mem::transmute::<u8, ProgramInstructions>(value) }),
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}

/// Trait used for struct which are instruction data
pub trait InstructionData { }

/// Copies the instruction type and data as bytes to the provided buffer slice.
/// Buf must be of sufficient length.
pub fn instruction_to_bytes<'a, T>(
    buf: &mut [MaybeUninit<u8>], 
    inst_type: &'a ProgramInstructions, 
    inst_data: &'a T) -> Result<(), ProgramError> 
where T: Transmutable + InstructionData, {
    if buf.len() < ProgramInstructions::LEN + T::LEN {
        return Err(CustomError::TransmutableError.into()); 
    }

    unsafe {
        copy_nonoverlapping(
            &[inst_type].as_ptr(), 
            buf.as_mut_ptr() as _, 
            size_of::<ProgramInstructions>());
    }

    unsafe {
        copy_nonoverlapping(
            inst_data.as_bytes().as_ptr(),
            buf.as_mut_ptr().add(size_of::<ProgramInstructions>()) as _, 
            size_of::<T>());
    }

    Ok(())
}
