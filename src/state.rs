use borsh::BorshSerialize;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::program_pack::IsInitialized;
use solana_sdk::sysvar::slot_history::ProgramError;

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

impl User {
    pub fn new(
        account_address: Pubkey,
        name: String,
        profile_photo: String,
        bio: String,
        friends: Vec<Pubkey>,
        posts: Vec<Pubkey>,
        followers: Vec<Pubkey>,
        followings: Vec<Pubkey>,
        views: usize,
    ) -> Self {
        Self {
            account_address,
            name,
            profile_photo,
            bio,
            friends,
            posts,
            followers,
            followings,
            views,
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, ProgramError> {
        self.try_to_vec()
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(data)
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn is_initialized(&self) -> bool {
        self.account_address != Pubkey::default()
    }

    pub fn add_following(&mut self, user_pubkey: Pubkey) {
        self.followings.push(user_pubkey);
    }

    pub fn add_follower(&mut self, follower_pubkey: Pubkey) {
        self.followers.push(follower_pubkey);
    }
}

impl UserAccount {
    pub fn new(user: User) -> Self {
        Self {
            is_initialized: true,
            user,
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, ProgramError> {
        self.try_to_vec()
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(data)
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

// impl IsInitialized for User {
//     fn is_initialized(&self) -> bool {
//         self.account_address != Pubkey::default()
//     }
// }

// impl IsInitialized for UserAccount {
//     fn is_initialized(&self) -> bool {
//         self.is_initialized
//     }
// }
