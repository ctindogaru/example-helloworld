use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use solana_program::{
  account_info::{AccountInfo},
  entrypoint::ProgramResult,
  msg,
  pubkey::Pubkey,
};
use spl_token::processor::Processor;


impl FutureAdmin {

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
}
