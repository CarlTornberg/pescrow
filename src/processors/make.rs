
use pinocchio::{AccountView, ProgramResult, error::ProgramError};

use crate::{helpers::bytes_helpers::from_bytes, states::escrow::Escrow};

pub(crate) fn process_make(inst_data: &[u8], accounts: &[AccountView]) -> ProgramResult {

    //      INSTRUCTION DATA
    // Extract instruction data
    // Validate instruction data
    let _inst_data = from_bytes::<MyInstructionData>(inst_data)?;

    // assert_eq!(inst_data.field_a(), 0u64);
    // assert_eq!(inst_data.field_b(), 0.0f32);
    
    //      ACCOUNTS
    // Extract accounts
    // Validate accounts
    // Deserialize accounts
    let [authority_view, data_view, _remaining @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    if !authority_view.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    //      BUSINESS LOGIC
    let escrow = from_bytes::<Escrow>(unsafe { &*(data_view.data_ptr() as *const [u8; Escrow::LEN]) })?;
    // assert_eq!(my_state.field_a(), 0);


    Ok(())
}
