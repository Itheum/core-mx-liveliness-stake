#![no_std]

use contexts::base::StorageCache;

multiversx_sc::imports!();

pub mod admin;
pub mod config;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod liveliness_stake_proxy;
pub mod rewards;
pub mod storage;

#[multiversx_sc::contract]
pub trait CoreMxLivelinessStake:
    admin::AdminModule
    + config::ConfigModule
    + events::EventsModule
    + rewards::RewardsModule
    + storage::StorageModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();

        let mut storage_cache = StorageCache::new(self);

        self.generate_aggregated_rewards(&mut storage_cache);

        let rewards = self.calculate_caller_share_in_rewards(&caller, &mut storage_cache, None);

        self.send().direct_non_zero_esdt_payment(
            &caller,
            &EsdtTokenPayment::new(self.rewards_token_identifier().get(), 0u64, rewards),
        );
    }
}
