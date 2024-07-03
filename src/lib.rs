#![no_std]

multiversx_sc::imports!();

pub mod liveliness_stake_proxy;

#[multiversx_sc::contract]
pub trait CoreMxLivelinessStake {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
