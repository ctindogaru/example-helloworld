use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum FutureContractError {
    /// Should be used when the entrypoint of the program receives an instruction
    /// that is not handled by the program.
    #[error("Instruction not handled")]
    InstructionNotHandled,
    /// Should be used when a person tries to redeem tokens from a future contract,
    /// but the maturity date of the contract is not reached.
    #[error("Maturity date of the future contract is not reached")]
    MaturityDateNotReached,
}
impl From<FutureContractError> for ProgramError {
    fn from(e: FutureContractError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for FutureContractError {
    fn type_of() -> &'static str {
        "FutureContractError"
    }
}
