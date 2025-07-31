use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    example_mocks::solana_sdk::system_instruction, program_error::ProgramError, pubkey::Pubkey,program::invoke
};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum NameInstruction {
    Initialize(String),
    Update(String),
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Data {
    name: String,
}

fn program_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult{
    let [payer, name_account, system_program_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    match NameInstruction::try_from_slice(instruction_data)? {
        NameInstruction::Initialize(name) => {
            let create_ix = system_instruction::create_account(
                payer.key,
                name_account.key,
                1000000000,
                82,
                program_id,
            );
            invoke(
                &create_ix,
                &[
                    payer.clone(),
                    name_account.clone(),
                    system_program_info.clone(),
                ],
            )?;

            let name_account_data = Data { name };
            name_account_data.serialize(&mut *name_account.data.borrow_mut())?;
        }

        NameInstruction::Update(name) => {
            let mut name_account_data = Data::try_from_slice(&name_account.data.borrow())?;
            name_account_data.name = name;
            name_account_data.serialize(&mut *name_account.data.borrow_mut())?;
        }
    }

    Ok(())
}
