#![no_std]

multiversx_sc::imports!();

pub mod admin;
pub mod config;
pub mod errors;
pub mod events;
pub mod liveliness_stake_proxy;

#[multiversx_sc::contract]
pub trait CoreMxLivelinessStake:
    admin::AdminModule + config::ConfigModule + events::EventsModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
