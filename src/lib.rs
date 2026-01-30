#![no_std]
pub mod helpers;
pub mod interface;
pub mod states; 
pub mod errors;
pub mod types;
mod processors;
use pinocchio::{
    AccountView, 
    Address, 
    ProgramResult, 
    entrypoint, 
    error::ProgramError, 
    hint::unlikely, nostd_panic_handler
};
use solana_program_log::log;
use crate::interface::ProgramInstructions;

solana_address::declare_id!("GJJuYV5QA1Lt9Ht5rdmVgvXdgjTJDe7nfJQ47YLvdstV");

nostd_panic_handler!();
entrypoint!(process_instruction);

pub fn process_instruction(
  program_id: &Address,
  accounts: &[AccountView],
  instruction_data: &[u8],
) -> ProgramResult {
    if unlikely(!address_eq(program_id, &crate::ID)) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let [discr, instruction_data @ ..] = instruction_data else {
        return Err(pinocchio::error::ProgramError::InvalidInstructionData);
    };

    match (*discr).try_into()? {
        ProgramInstructions::MyInstruction => {
            log!("My instruction.");
            processors::process_my_instruction(instruction_data, accounts)
        },
    }
}
