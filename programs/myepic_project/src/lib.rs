use anchor_lang::prelude::*;

declare_id!("8BmWcSGjjh91EhH4WM3JYwXgfSxfCti8esQpsHKq77E4");

#[program]
pub mod myepic_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
