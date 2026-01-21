pub mod my_instruction;

use core::ptr::copy_nonoverlapping;

pub use my_instruction::*;
use pinocchio::error::ProgramError;

use crate::{states::{Transmutable, as_bytes}, types::Discriminator};

#[repr(u8)]
pub enum ProgramInstructions {
    MyInstruction,
}

/// SAFETY Is a single byte in size
unsafe impl Transmutable for ProgramInstructions { 
    const LEN: usize = size_of::<Self>();
}

impl Into<Discriminator> for ProgramInstructions {
    fn into(self) -> Discriminator {
        self as Discriminator
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
            0..=1 => Ok(unsafe { core::mem::transmute::<u8, ProgramInstructions>(value) }),
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}

/// Trait used for struct which are an instruction data
pub trait ProgramInstruction {}

/// Copies the instruction type and data as bytes to the provided buffer slice.
/// Buf must be of sufficient length.
pub fn instruction_to_bytes<'a, T>(buf: &mut [u8], inst_type: &'a ProgramInstructions, inst_data: &'a T) -> Result<(), ProgramError>
where T: Transmutable + ProgramInstruction, {
    if buf.len() < ProgramInstructions::LEN + T::LEN {
        return Err(ProgramError::BorshIoError); // TODO Change to a better error?
    }

    unsafe {
        copy_nonoverlapping(
            &[inst_type].as_ptr(), 
            buf.as_mut_ptr() as _, 
            size_of::<ProgramInstructions>());
    }

    unsafe {
        copy_nonoverlapping(
            as_bytes(inst_data).as_ptr(),
            buf.as_mut_ptr().add(size_of::<ProgramInstructions>()) as _, 
            size_of::<T>());
    }

    Ok(())

}
