use std::ops::{Div, Mul};
use anchor_lang::prelude::*;
use crate::errors::CustomError;
pub fn convert_to_float(value: u64, decimals: u8) -> f64 {
    (value as f64).div(f64::powf(10.0, decimals as f64))
}

pub fn convert_from_float(value: f64, decimals: u8) -> u64 {
    value.mul(f64::powf(10.0, decimals as f64)) as u64
}



pub fn linear_buy_cost(amount: u64, reserve_ratio: u16, total_supply: u64) -> Result<u64> {

        let new_supply = total_supply
            .checked_add(amount)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        let new_supply_squared = (new_supply as u128)
            .checked_mul(new_supply as u128)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        let total_supply_squared = (total_supply as u128)
            .checked_mul(total_supply as u128)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        let numerator = new_supply_squared
            .checked_sub(total_supply_squared)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?
            .checked_div(2)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;


        let denominator = (reserve_ratio as u128)
            .checked_mul(10000)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;


        let cost = numerator
            .checked_div(denominator)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        if cost > u64::MAX as u128 {
            return Err(CustomError::OverFlowUnderFlowOccured.into());
        }

        Ok(cost as u64)

}


pub fn linear_sell_cost(amount: u64, reserve_ratio: u16, total_supply: u64) -> Result<u64> {

    if amount > total_supply {
        return Err(CustomError::InsufficientBalance.into());
    }

    let new_supply = total_supply
        .checked_sub(amount)
        .ok_or(CustomError::OverFlowUnderFlowOccured)?;

    let total_supply_squared = (total_supply as u128)
        .checked_mul(total_supply as u128)
        .ok_or(CustomError::OverFlowUnderFlowOccured)?;

    let new_supply_squared = (new_supply as u128)
        .checked_mul(new_supply as u128)
        .ok_or(CustomError::OverFlowUnderFlowOccured)?;

    let numerator = total_supply_squared
        .checked_sub(new_supply_squared)
        .ok_or(CustomError::OverFlowUnderFlowOccured)?
        .checked_div(2)
        .ok_or(CustomError::OverFlowUnderFlowOccured)?;

    let denominator = (reserve_ratio as u128)
        .checked_mul(10000)
        .ok_or(CustomError::OverFlowUnderFlowOccured)?;

    let reward = numerator
        .checked_div(denominator)
        .ok_or(CustomError::OverFlowUnderFlowOccured)?;

    if reward > u64::MAX as u128 {
        return Err(CustomError::OverFlowUnderFlowOccured.into());
    }
    
    Ok(reward as u64)

}


pub fn quadratic_buy_cost(amount: u64, reserve_ratio: u16, total_supply: u64) -> Result<u64> {
    // TODO: Implement quadratic buy cost
    Ok(0)
}

pub fn quadratic_sell_cost(amount: u64, reserve_ratio: u16, total_supply: u64) -> Result<u64> {
    // TODO: Implement quadratic sell cost
    Ok(0)
}
