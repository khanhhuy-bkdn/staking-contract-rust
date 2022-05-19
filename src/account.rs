use near_sdk::Timestamp;
use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Account {
    pub stake_balance: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub unstake_balance: Balance,
    pub unstake_start_timestamp: Timestamp,
    pub unstake_available_epoch: EpochHeight
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum UpgradableAccount {
    Current(Account)
}

impl From<UpgradableAccount> for Account {
    fn from(upgradable_account: UpgradableAccount) -> Self {
        match upgradable_account {
            UpgradableAccount::Current(account) => account,
        }
    }
}

impl From<Account> for UpgradableAccount {
    fn from(account: Account) -> Self {
        UpgradableAccount::Current(account)
    }
}