#![no_std]

pub mod interface;
pub mod states; 
mod processors;
pub mod types;
use core::mem::MaybeUninit;


use pinocchio::{
  AccountView, Address, ProgramResult, entrypoint, error::ProgramError, hint::unlikely
};

solana_address::declare_id!("GJJuYV5QA1Lt9Ht5rdmVgvXdgjTJDe7nfJQ47YLvdstV");


#[inline(always)]
pub fn write_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {

    // SAFETY:
    // - Pointers are of alignment 1,
    // - the length will not exceed either pointers length
    unsafe {
        core::ptr::copy_nonoverlapping(
            source.as_ptr(), 
            destination.as_mut_ptr() as *mut u8, 
            destination.len().min(source.len())
        );
    }
}

use solana_address::address_eq;
use solana_program_log::log;

use crate::interface::ProgramInstructions;


entrypoint!(process_instruction);

pub fn process_instruction(
  program_id: &Address,
  accounts: &[AccountView],
  instruction_data: &[u8],
) -> ProgramResult {
    if unlikely(address_eq(program_id, &crate::ID)) {
        return Err(ProgramError::IncorrectProgramId);
    }

    log("Hello from my pinocchio program!");
    let [discr, instruction_data @ ..] = instruction_data else {
        return Err(pinocchio::error::ProgramError::InvalidInstructionData);
    };

    match (*discr).try_into()? {
        ProgramInstructions::MyInstruction => {
            log!("My instruction.");
            processors::process_something_processor(instruction_data, accounts)
        },
    }
}
