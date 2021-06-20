use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use solana_program::{
  account_info::AccountInfo,
  entrypoint::ProgramResult,
  msg,
  pubkey::Pubkey
};

use crate::error::FutureContractError;

use spl_token::instruction::TokenInstruction;
use spl_token::processor::Processor as SplTokenProcessor;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FutureContract {
  locked_token_contract_address: String, // the contract address of the token that's going to be locked in the future
  future_expiration: u16, // when is the future set to expire (in days)
  future_creation_date: u64, // creation date of the future as a unix timestamp
  future_contract_address: String // the contract address of the future
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    // locked token contract address to all its associated futures
    pub tokens: HashMap<String, Vec<String>>,
    // future contract address to future contract data
    pub futures: HashMap<String, FutureContract>
}

pub struct Processor { }

impl Processor {
  pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let instruction = TokenInstruction::unpack(input)?;

    match instruction {
      // TODO: add instruction for creating the token
      TokenInstruction::MintTo { amount } => {
        msg!("Instruction: MintTo");
        Self::mint_into_future_contract(program_id, accounts, amount)
      }
      TokenInstruction::Burn { amount } => {
        msg!("Instruction: Burn");
        Self::burn_from_future_contract(program_id, accounts, amount)
      }
      _ => {
        msg!("Instruction: Not handled");
        return Err(FutureContractError::InstructionNotHandled.into())
      }
    }
  }

  fn mint_into_future_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64
  ) -> ProgramResult {
    // TODO: include useful data in the log (ex: program_id, accounts, amount)
    msg!("MINT instruction triggered");
    SplTokenProcessor::process_mint_to(program_id, accounts, amount, None)
  }

  fn burn_from_future_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64
  ) -> ProgramResult {
    // TODO: replace this with the actual future contract address
    // (it should be part of the 'accounts' variable)
    let future_contract_address = Pubkey::new_unique();
    // TODO: replace this with the futures data stored in 'accounts'
    // (it should be part of the 'accounts' variable)
    let futures: HashMap<Pubkey, FutureContract> = HashMap::new();

    let future_data = futures.get(&future_contract_address).unwrap();

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let days_diff = (current_time - future_data.future_creation_date) / 86400; // 1day = 86400s
    if days_diff < future_data.future_expiration as u64 {
      return Err(FutureContractError::MaturityDateNotReached.into());
    }

    // TODO: include useful data in the log (ex: program_id, accounts, amount)
    msg!("BURN instruction triggered");
    SplTokenProcessor::process_burn(program_id, accounts, amount, None)
  }
}
