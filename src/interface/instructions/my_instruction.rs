use core::slice::from_raw_parts;

use pinocchio::{
    AccountView, 
    ProgramResult, 
    cpi::{Signer, invoke_signed}, 
    instruction::{InstructionAccount, InstructionView}};

use crate::{
    helpers::bytes_helpers::Transmutable, 
    interface::{InstructionData, ProgramInstructions, instruction_to_bytes}, 
    types::{DISCRIMINATOR_LEN, F32Bytes, U64Bytes, UNINIT_BYTE} 
};

pub struct MyInstruction<'a> {
    /// To Account
    pub to: &'a AccountView,

    /// From account
    pub from: &'a AccountView,

    /// Instruction data
    pub data: &'a MyInstructionData,
}

impl MyInstruction<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
       self.invoke_signed(&[]) 
    }

    #[inline]
    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        const INST_LEN: usize = DISCRIMINATOR_LEN + MyInstructionData::LEN;

        // Instruction accounts
        let inst_accs: [InstructionAccount; 2] = [
            InstructionAccount::writable_signer(self.from.address()),
            InstructionAccount::writable(self.to.address())
        ];
        
        let mut inst_data = [UNINIT_BYTE; INST_LEN];
        instruction_to_bytes(
            &mut inst_data, 
            &ProgramInstructions::MyInstruction, 
            self.data)?;
        
        // Create instruction
        let inst = InstructionView {
            program_id: &crate::ID,
            accounts: &inst_accs,
            data: unsafe { from_raw_parts(inst_data.as_ptr() as _, INST_LEN) }
        };

        // Invoke instruction
        invoke_signed(
            &inst, 
            &[self.to, self.from],
            signers,
        )
    }
}

impl InstructionData for MyInstructionData { }

#[repr(C)]
pub struct MyInstructionData {
    field_a: U64Bytes,
    field_b: F32Bytes,
}

// SAFETY: Struct is only of u8's.
unsafe impl Transmutable for MyInstructionData { 
    const LEN: usize = size_of::<Self>();
}

impl MyInstructionData {
    pub fn new(field_a: u64, field_b: f32) -> Self {
        Self { 
            field_a: field_a.to_le_bytes(), 
            field_b: field_b.to_le_bytes() 
        }
    }

    pub fn field_a(&self) -> u64 {
        u64::from_le_bytes(self.field_a)
    }

    pub fn field_b(&self) -> f32 {
        f32::from_le_bytes(self.field_b)
    }
}

#[cfg(test)]
mod solana_sdk {
    #![allow(unused)]
    //! Helper functions to interact with the solana SDK.

    use solana_sdk::{message::AccountMeta};
    use crate::helpers::Transmutable;

    pub struct MyInstruction<'a> {
        accounts: super::MyInstruction<'a>,
        data: super::MyInstructionData,
    }

    impl MyInstruction<'_> {
        pub fn to_solana_sdk_instruction(&self) -> solana_sdk::instruction::Instruction {
            solana_sdk::instruction::Instruction {
                program_id: crate::ID,
                accounts: [
                    AccountMeta::new(
                        solana_address::Address::new_from_array(
                            self.accounts.from.address().to_bytes()), 
                        true),
                    AccountMeta::new(
                        solana_address::Address::new_from_array(
                            self.accounts.to.address().to_bytes()), 
                        true)

                ].to_vec(),
                data: self.data.as_bytes().to_vec(),
            }
        }
    }
}
