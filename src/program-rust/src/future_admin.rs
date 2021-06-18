use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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

struct FutureContract {
  locked_token_contract_address: Pubkey, // the contract address of the token that's going to be locked in the future
  future_expiration: u16, // when is the future set to expire (in days)
  future_creation_date: u64, // creation date of the future as a unix timestamp
  future_contract_address: Pubkey // the contract address of the future
}

struct FutureAdmin {
  tokens: HashMap<Pubkey, Vec<Pubkey>>, // locked token contract address to all its associated futures
  futures: HashMap<Pubkey, FutureContract> // future contract address to future contract data
}

impl FutureAdmin {
  fn new() -> FutureAdmin {
    FutureAdmin {
      tokens: HashMap::new(),
      futures: HashMap::new()
    }
  }

  fn create_future_contract(&mut self, locked_token_contract_address: Pubkey, future_expiration: u16, locked_amount: u64) {
    // check if the token has existing futures
    if self.tokens.get(&locked_token_contract_address).is_some() {
      let last_future = self.tokens.get(&locked_token_contract_address).unwrap().last().unwrap();
      let last_future_data = self.futures.get(&last_future).unwrap();

      let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
      let days_diff = (current_time - last_future_data.future_creation_date) / 86400; // 1day = 86400s
      if days_diff < 30 {
        panic!("MonoFutures: futureCooldownNotElapsed");
      }
    }

    // TODO: change this to be the address of the future contract
    let future_contract_address = locked_token_contract_address;

    let new_future = FutureContract {
      locked_token_contract_address: locked_token_contract_address,
      future_expiration: future_expiration,
      future_creation_date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
      future_contract_address: future_contract_address
    };

    if self.tokens.get(&locked_token_contract_address).is_none() {
      self.tokens.insert(
        locked_token_contract_address,
        Vec::new()
      );
    }
    self.tokens.get_mut(&locked_token_contract_address).unwrap().push(future_contract_address);

    self.futures.insert(
      future_contract_address,
      new_future
    );

    msg!("Created Future Contract Address: {} for Token Contract Address: {}", future_contract_address, locked_token_contract_address);

    if locked_amount > 0 {
      Self::mint_into_future_contract(&self, future_contract_address, locked_amount);
    }
  }

  fn mint_into_future_contract(&self, future_contract_address: Pubkey, locked_amount: u64) {
    // TODO: mint the locked_amount into the future_contract_address

    msg!("Minted {} amount into Future Contract Address: {}", locked_amount, future_contract_address);
  }

  fn burn_from_future_contract(&self, future_contract_address: Pubkey, locked_amount: u64) {
    let future_data = self.futures.get(&future_contract_address).unwrap();

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let days_diff = (current_time - future_data.future_creation_date) / 86400; // 1day = 86400s
    if days_diff < future_data.future_expiration as u64 {
      panic!("MonoFutures: maturityDateNotReached");
    }

    // TODO: burn the locked_amount from the future_contract_address

    msg!("Burned {} amount from Future Contract Address: {}", locked_amount, future_contract_address);
  }
}
