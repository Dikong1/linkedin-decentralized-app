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

pub struct Context<'a> {
    pub program_id: &'a Pubkey,
    pub accounts: &'a [AccountInfo<'a>],
}

impl<'a> Context<'a> {
    pub fn new(program_id: &'a Pubkey, accounts: &'a [AccountInfo<'a>]) -> Self {
        Self { program_id, accounts }
    }
}

use crate::{
    instructions::{process_add_user, process_follow_user}, // Import instructions module
    state::{User, UserAccount}, // Import state module
};

pub mod instructions;
pub mod state;

entrypoint!(process_instruction); // Define the entrypoint function

pub fn process_instruction<'a, 'b: 'a>(
    program_id: &'b Pubkey, // The program's ID
    accounts: &'a [AccountInfo<'a>], // Accounts associated with the program
    instruction_data: &[u8], // Instruction data passed to the program
) -> ProgramResult {
    // Decode instruction data to determine the action
    let instruction = match instruction_data.get(0) {
        Some(&1) => Instruction::AddUser,
        Some(&2) => Instruction::FollowUser,
        _ => return Err(ProgramError::InvalidInstructionData), // Invalid instruction
    };

    // Handle the different instructions
    match instruction {
        Instruction::AddUser => {
            // Extract data from accounts and perform add user logic
            let ctx = Context::new(program_id, accounts);
            let name = String::from_utf8(instruction_data[1..33].to_vec()).unwrap(); // Assuming name is passed as bytes
            let profile_photo = String::from_utf8(instruction_data[33..65].to_vec()).unwrap(); // Assuming profile photo is passed as bytes
            let bio = String::from_utf8(instruction_data[65..97].to_vec()).unwrap(); // Assuming bio is passed as bytes
            process_add_user(ctx.accounts, name, profile_photo, bio)
        }
        Instruction::FollowUser => {
            // Extract data from accounts and perform follow user logic
            let ctx = Context::new(program_id, accounts);
            let user_to_follow_pubkey = Pubkey::new(&instruction_data[1..33]);
            process_follow_user(ctx.accounts, user_to_follow_pubkey)
        }
    }
}

// Define an enum to represent different instructions
enum Instruction {
    AddUser,
    FollowUser,
}
