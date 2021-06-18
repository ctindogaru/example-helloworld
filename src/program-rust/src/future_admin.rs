use std::collections::HashMap;

use solana_program::{
  account_info::{AccountInfo},
  entrypoint::ProgramResult,
  msg,
  pubkey::Pubkey,
};
use spl_token::processor::Processor;

// Program entrypoint's implementation
pub fn process_future_instruction(
  program_id: &Pubkey, // Public key of the account the hello world program was loaded into
  accounts: &[AccountInfo], // The account to say hello to
  _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
  msg!("Entered in future instruction");
  msg!("program_id: {}, accounts: {:?}, _instruction_data: {:?}", program_id, accounts, _instruction_data);
  
  Ok(())
}

enum FutureType {
  MonoF90,
  MonoF180,
  MonoF270,
  MonoF360,
  MonoF450,
  MonoF540,
  MonoF630,
  MonoF720
}

struct FutureContract {
  locked_token_contract_address: Pubkey,
  future_type: FutureType,
  future_creation_date: u64,
  future_contract_address: Pubkey
}

struct FutureAdmin {
  tokens: HashMap<Pubkey, Vec<FutureContract>>,
  futures: HashMap<Pubkey, FutureContract>
}

impl FutureAdmin {
  fn create_future_contract(locked_token_contract_address: Pubkey, future_type: FutureType, locked_amount: u64) {

  }

  fn mint_into_future_contract(future_contract_address: Pubkey, locked_amount: u64) {

  }

  fn burn_from_future_contract(future_contract_address: Pubkey, locked_amount: u64) {

  }
}
