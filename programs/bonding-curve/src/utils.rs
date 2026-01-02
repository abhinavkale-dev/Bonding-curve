pub const BPS_DENOMINATOR: u64 = 10_000;
pub const MAX_PRICE_IMPACT_BPS: u64 = 1_000;

pub fn calculate_fees(amount: u64, fee_bps: u64) -> u64 {
    amount.saturating_mul(fee_bps)/ BPS_DENOMINATOR
}

pub fn calculate_tokens_out(
    sol_amount: u64,
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
) -> u64 {
    let k = virtual_sol_reserves.saturating_mul(virtual_token_reserves);

    let new_virtual_token_reserves = k/ virtual_sol_reserves.saturating_add(sol_amount);

    virtual_token_reserves.saturating_sub(new_virtual_token_reserves)
}

pub fn calculate_sol_out(
    tokens_amount: u64,
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
) -> u64 {
    let k = virtual_sol_reserves.saturating_mul(virtual_token_reserves);

    let new_virtual_sol_reserves = k/ virtual_token_reserves.saturating_add(tokens_amount);

    virtual_sol_reserves.saturating_sub(new_virtual_sol_reserves)
}

pub fn price_impact_bps(
    amount_out: u64,
    reserve: u64,
) -> u64 {
    amount_out.saturating_mul(BPS_DENOMINATOR)/ reserve
}

pub fn enforce_price_impact(
    amount_out: u64,
    reserve: u64,
) -> bool {
    price_impact_bps(amount_out, reserve) <= MAX_PRICE_IMPACT_BPS
}