use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};
use spl_token::error::TokenError;

mod error;
mod processor;
use processor::Processor;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        // TODO: the type can be either TokenError or FutureContractError. Solve the issue.
        error.print::<TokenError>();
        return Err(error);
    }
    Ok(())
}
