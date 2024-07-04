#![no_std]

use contexts::base::StorageCache;
use core_mx_life_bonding_sc::errors::ERR_CONTRACT_NOT_READY;

multiversx_sc::imports!();

pub mod admin;
pub mod config;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod liveliness_stake_proxy;
pub mod rewards;
pub mod storage;
pub mod views;

#[multiversx_sc::contract]
pub trait CoreMxLivelinessStake:
    admin::AdminModule
    + config::ConfigModule
    + events::EventsModule
    + rewards::RewardsModule
    + storage::StorageModule
    + views::ViewsModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);

        let caller = self.blockchain().get_caller();

        let mut storage_cache = StorageCache::new(self);

        self.generate_aggregated_rewards(&mut storage_cache);

        let rewards = self.calculate_caller_share_in_rewards(&caller, &mut storage_cache, false);

        storage_cache.accumulated_rewards -= &rewards;

        self.send().direct_non_zero_esdt_payment(
            &caller,
            &EsdtTokenPayment::new(self.rewards_token_identifier().get(), 0u64, rewards),
        );
    }
}
