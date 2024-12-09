use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,entrypoint, pubkey::Pubkey
};

mod processor;
mod instruction;
mod state;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult{
    processor::Processor::process_instruction(program_id, accounts, instruction_data)
}
