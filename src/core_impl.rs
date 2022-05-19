use near_sdk::{ext_contract, Gas, PromiseResult};

use crate::*;

pub const DEPOSIT_ONE_YOCTO: Balance = 1;
pub const NO_DEPOSIT: Balance = 0;
pub const FT_TRANSFER_GAS: Gas = 10_000_000_000_000;
pub const FT_HARVEST_CALLBACK_GAS: Gas = 10_000_000_000_000;

pub trait FungibleTokenReceiver {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl FungibleTokenReceiver for StakingContract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        self.internal_deposit_and_stake(sender_id, amount.0);

        PromiseOrValue::Value(U128(0))
    }
}