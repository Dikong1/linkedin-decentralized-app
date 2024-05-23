use solana_program::borsh::{BorshDeserialize, BorshSerialize}; // Import Borsh traits directly

#[derive(Clone, Debug, Default, BorshSerialize, BorshDeserialize)]
pub struct User {
    pub account_address: Pubkey,
    pub name: String,
    pub profile_photo: String,
    pub bio: String,
    pub friends: Vec<Pubkey>,
    pub posts: Vec<Pubkey>,
    pub followers: Vec<Pubkey>,
    pub followings: Vec<Pubkey>,
    pub views: usize,
}

#[derive(Clone, Debug, Default, BorshSerialize, BorshDeserialize)]
pub struct UserAccount {
    pub is_initialized: bool,
    pub user: User,
}

impl Sealed for User {}

impl Sealed for UserAccount {}

impl IsInitialized for User {
    fn is_initialized(&self) -> bool {
        self.account_address != Pubkey::default()
    }
}

impl IsInitialized for UserAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for User {
    const LEN: usize = 224; // Adjust the length as per your fields
    fn pack_into_slice(&self, output: &mut [u8]) {
        let mut writer = std::io::Cursor::new(output);
        self.serialize(&mut writer).unwrap();
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let mut reader = std::io::Cursor::new(input);
        Ok(Self::deserialize(&mut reader).unwrap())
    }
}

impl Pack for UserAccount {
    const LEN: usize = User::LEN + 1; // Add 1 for is_initialized field
    fn pack_into_slice(&self, output: &mut [u8]) {
        let mut writer = std::io::Cursor::new(output);
        self.serialize(&mut writer).unwrap();
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let mut reader = std::io::Cursor::new(input);
        Ok(Self::deserialize(&mut reader).unwrap())
    }
}
