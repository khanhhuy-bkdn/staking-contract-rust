use near_sdk::env::block_index;

use crate::*;

#[near_bindgen]
impl StakingContract {
    pub(crate) fn internal_register_account(&mut self, account_id: AccountId) {
        let account = Account {
            stake_balance: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            unstake_balance: 0,
            unstake_start_timestamp: 0,
            unstake_available_epoch: 0,
        };

        self.accounts
            .insert(&account_id, &UpgradableAccount::from(account));
    }

    pub(crate) fn internal_deposit_and_stake(&mut self, account_id: AccountId, amount: u128) {
        let upgrade_account = self.accounts.get(&account_id);
        assert!(upgrade_account.is_some(), "ERR_ACCOUNT_NOT_FOUND");
        assert_eq!(self.paused, false, "ERR_CONTRACT_PAUSED");
        assert_eq!(
            self.ft_contract_id,
            env::predecessor_account_id(),
            "ERR_INVALID_FT_CONTRACT_ID"
        );

        let mut account = Account::from(upgrade_account.unwrap());
        if account.stake_balance == 0 {
            self.total_staker += 1;
        }
        let new_reward = self.internal_calculator_reward_account(account);

        account.pre_reward += new_reward;
        account.last_block_balance_change = env::block_index();
        account.stake_balance += amount;

        self.accounts
            .insert(&account_id, &UpgradableAccount::from(account));

        let new_contract_reward = self.internal_calculator_reward_global();

        self.pre_reward += new_contract_reward;
        self.last_block_balance_change = env::block_index();
        self.total_stake_balance += amount;
    }

    pub(crate) fn internal_calculator_reward_account(&self, account: Account) -> Balance {
        let last_block = if self.paused {
            self.pause_in_block
        } else {
            env::block_index()
        };
        let diff_block = last_block - account.last_block_balance_change;
        let reward = (account.stake_balance
            * self.config.reward_numerator as u128
            * diff_block as u128) as u128
            / self.config.reward_denumerator as u128;
        reward
    }

    pub(crate) fn internal_calculator_reward_global(&self) -> Balance {
        let last_block = if self.paused {
            self.pause_in_block
        } else {
            env::block_index()
        };
        let diff_block = last_block - self.last_block_balance_change;
        let reward = (self.total_stake_balance
            * self.config.reward_numerator as u128
            * diff_block as u128) as u128
            / self.config.reward_denumerator as u128;
        reward
    }
}
