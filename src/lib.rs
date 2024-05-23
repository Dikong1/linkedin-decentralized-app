use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
};

use crate::{
    instructions::{process_add_user, process_follow_user}, // Import instructions module
    state::{User, UserAccount}, // Import state module
};

pub mod instructions;
pub mod state;

#[entrypoint]
pub fn entry(_ctx: Context, _accounts: &[AccountInfo], _instruction_data: &[u8]) -> ProgramResult {
    msg!("Hello from Solana program");
    Ok(())
}

#[entrypoint]
pub fn add_user(
    ctx: Context<AddUser>,
    name: String,
    profile_photo: String,
    bio: String,
) -> ProgramResult {
    process_add_user(ctx.accounts, name, profile_photo, bio)
}

#[entrypoint]
pub fn follow_user(
    ctx: Context<FollowUser>,
    user_to_follow_pubkey: Pubkey,
) -> ProgramResult {
    process_follow_user(ctx.accounts, user_to_follow_pubkey)
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_entry() {
        let mut program_test = ProgramTest::new(
            "my_program",
            id!(),
            processor!(entry),
        );
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_bytes(id!(), &[], vec![])],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert!(banks_client.process_transaction(transaction).await.is_ok());
    }
}
