use anchor_lang::prelude::*;

declare_id!("Hfd7V12kj9AENQjLpTozaPW6aT2rhPm3LSyjXZ5AbWH");

#[program]
pub mod anchor_escrow {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
