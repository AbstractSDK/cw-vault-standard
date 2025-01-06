use cosmwasm_std::Coin;
pub use cosmwasm_std_1::Coin as Coin1;

pub mod traits;

fn coin2_to_coin1(coin: &Coin) -> Coin1 {
    Coin1::new(coin.amount.u128(), coin.denom.clone())
}

fn coins2_to_coins1<'a>(coins: impl IntoIterator<Item = &'a Coin>) -> Vec<Coin1> {
    coins.into_iter().map(coin2_to_coin1).collect()
}
