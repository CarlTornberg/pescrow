use pinocchio::{AccountView, ProgramResult, error::ProgramError};

use crate::{helpers::{Transmutable, bytes_helpers::from_bytes}, interface::MyInstructionData, states::my_state::MyState};

pub(crate) fn process_my_instruction(inst_data: &[u8], accounts: &[AccountView]) -> ProgramResult {

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
    let _my_state = from_bytes::<MyState>(unsafe { &*(data_view.data_ptr() as *const [u8; MyState::LEN]) })?;
    // assert_eq!(my_state.field_a(), 0);


    Ok(())
}
