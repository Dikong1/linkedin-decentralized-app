use crate::state::{User, UserAccount}; // Import state module
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};

pub fn process_add_user(
    accounts: &[AccountInfo],
    name: String,
    profile_photo: String,
    bio: String,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let user_account_info = next_account_info(accounts_iter)?;
    let user_program_info = next_account_info(accounts_iter)?;

    // Check if the user account is initialized
    let mut user_account_data = user_account_info.try_borrow_mut_data()?;
    let user_account = UserAccount::deserialize(&user_account_data)?;

    if user_account.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Create a new user account
    let user = User::new(
        user_account_info.key.clone(), // Assuming account address is a Pubkey
        name,
        profile_photo,
        bio,
        Vec::new(), // Initialize friends list as empty
        Vec::new(), // Initialize posts list as empty
        Vec::new(), // Initialize followers list as empty
        Vec::new(), // Initialize followings list as empty
        0,          // Initialize views as 0
    );
    
    let mut new_user_account = UserAccount::new(user);

    
    // Serialize and save the new user account
    new_user_account.serialize()?;

    // Copy the user's key to the program's data account
    user_program_info.data.borrow_mut().copy_from_slice(&user_account_info.key.to_bytes());

    Ok(())
}

pub fn process_follow_user(
    accounts: &[AccountInfo],
    user_to_follow_pubkey: Pubkey,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let user_account_info = next_account_info(accounts_iter)?;
    let user_to_follow_account_info = next_account_info(accounts_iter)?;

    // Check if both accounts are initialized
    let mut user_account_data = user_account_info.try_borrow_mut_data()?;
    let user_account = UserAccount::deserialize(&user_account_data)?;

    if !user_account.is_initialized() {
        return Err(ProgramError::UninitializedAccount);
    }

    let mut user_to_follow_account_data = user_to_follow_account_info.try_borrow_mut_data()?;
    let user_to_follow_account = UserAccount::deserialize(&user_to_follow_account_data)?;

    if !user_to_follow_account.is_initialized() {
        return Err(ProgramError::UninitializedAccount);
    }

    // Update user's followings list
    let mut user = User::deserialize(&user_account_data)?;
    user.add_following(user_to_follow_pubkey);
    user.serialize()?;

    // Update user_to_follow's followers list
    let mut user_to_follow = User::deserialize(&user_to_follow_account_data)?;
    user_to_follow.add_follower(*user_account_info.key);
    user_to_follow.serialize()?;

    Ok(())
}
