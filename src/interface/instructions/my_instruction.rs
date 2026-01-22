use core::slice::from_raw_parts;

use pinocchio::{AccountView, ProgramResult, cpi::{Signer, invoke_signed}, instruction::{InstructionAccount, InstructionView}};

use crate::{
    interface::ProgramInstructions, 
    helpers::bytes_helpers::{Transmutable, write_bytes}, 
    types::{F32Bytes, U64Bytes, UNINIT_BYTE}, 
};

pub struct MyInstruction<'a> {
    /// To Account
    pub to: &'a AccountView,

    /// From account
    pub from: &'a AccountView,

    /// Amount
    pub amount: u64,
}

impl MyInstruction<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
       self.invoke_signed(&[]) 
    }

    #[inline]
    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        const INST_LEN: usize = 9;

        // Instruction accounts
        let inst_accs: [InstructionAccount; 2] = [
            InstructionAccount::writable_signer(self.from.address()),
            InstructionAccount::writable(self.to.address())
        ];
        
        // Instruction data
        // - [0]: discriminator
        // - [1..9]: amount (u64)
        let mut inst_data = [UNINIT_BYTE; INST_LEN];

        write_bytes(&mut inst_data, &[ProgramInstructions::MyInstruction.into()]);
        write_bytes(&mut inst_data[1..9], &self.amount.to_le_bytes());
        
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
