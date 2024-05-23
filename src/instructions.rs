use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    program_error::ProgramError,
};

use crate::state::{User, UserAccount}; // Import state module

pub fn process_add_user(
    accounts: &[AccountInfo],
    name: String,
    profile_photo: String,
    bio: String,
) -> ProgramResult {
    // Extract the account info
    let accounts_iter = &mut accounts.iter();
    let user_account_info = next_account_info(accounts_iter)?;
    let user_program_info = next_account_info(accounts_iter)?;

    // Check if the user account is already initialized
    let user_account = UserAccount::unpack_unchecked(&user_account_info.data.borrow())?;
    if user_account.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Create a new user account
    let mut user = User::new(user_account_info.key, name, profile_photo, bio);
    user.serialize(&mut &mut user_account_info.data.borrow_mut()[..])?;
    user_program_info
        .data
        .borrow_mut()
        .copy_from_slice(&user_account_info.key.to_bytes());

    Ok(())
}

pub fn process_follow_user(
    accounts: &[AccountInfo],
    user_to_follow_pubkey: Pubkey,
) -> ProgramResult {
    // Extract the account info
    let accounts_iter = &mut accounts.iter();
    let user_account_info = next_account_info(accounts_iter)?;
    let user_to_follow_account_info = next_account_info(accounts_iter)?;

    // Check if both accounts are initialized
    let user_account = UserAccount::unpack_unchecked(&user_account_info.data.borrow())?;
    if !user_account.is_initialized() {
        return Err(ProgramError::AccountNotInitialized);
    }

    let user_to_follow_account = UserAccount::unpack_unchecked(&user_to_follow_account_info.data.borrow())?;
    if !user_to_follow_account.is_initialized() {
        return Err(ProgramError::AccountNotInitialized);
    }

    // Update user's followings list
    let mut user = User::unpack_unchecked(&user_account_info.data.borrow())?;
    user.add_following(user_to_follow_pubkey);
    user.serialize(&mut &mut user_account_info.data.borrow_mut()[..])?;

    // Update user_to_follow's followers list
    let mut user_to_follow = User::unpack_unchecked(&user_to_follow_account_info.data.borrow())?;
    user_to_follow.add_follower(*user_account_info.key);
    user_to_follow.serialize(&mut &mut user_to_follow_account_info.data.borrow_mut()[..])?;

    Ok(())
}
