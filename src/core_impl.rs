use near_sdk::{ext_contract, Gas, PromiseResult};

use crate::*;

pub const DEPOSIT_ONE_YOCTO: Balance = 1;
pub const NO_DEPOSIT: Balance = 0;
pub const FT_TRANSFER_GAS: Gas = 10_000_000_000_000;
pub const FT_HARVEST_CALLBACK_GAS: Gas = 10_000_000_000_000;

pub trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[ext_contract(ext_ft_contract)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_self)]
pub trait ExtStakingContract {
    fn ft_withdraw_callback(&mut self, account_id: AccountId, old_account: Account);
}

#[near_bindgen]
impl FungibleTokenReceiver for StakingContract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        self.internal_deposit_and_stake(sender_id, amount.0);

        PromiseOrValue::Value(U128(0))
    }
}

#[near_bindgen]
impl StakingContract {
    #[payable]
    pub fn unstake(&mut self, amount: U128) {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        self.internal_unstake(account_id, amount.0);
    }

    #[payable]
    pub fn withdraw(&mut self) -> Promise {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        let old_account = self.internal_withdraw(account_id.clone());

        ext_ft_contract::ft_transfer(
            account_id.clone(),
            U128(old_account.unstake_balance),
            Some("Staking contract withdraw".to_string()),
            &self.ft_contract_id,
            DEPOSIT_ONE_YOCTO,
            FT_TRANSFER_GAS,
        )
        .then(ext_self::ft_withdraw_callback(
            account_id.clone(),
            old_account,
            &env::current_account_id(),
            NO_DEPOSIT,
            FT_HARVEST_CALLBACK_GAS,
        ))
    }

    #[private]
    pub fn ft_withdraw_callback(&mut self, account_id: AccountId, old_account: Account) -> U128 {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_valie) => {
                U128(old_account.unstake_balance)
            }
            PromiseResult::Failed => {
                self.accounts.insert(&account_id, &UpgradableAccount::from(old_account));
                U128(0)
            }
        }
    }
}
