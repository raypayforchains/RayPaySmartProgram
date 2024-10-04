use anchor_lang::prelude::*;


declare_id!("2FCmh5tZDnu7V1wFjwj6JSuGs2nVkBViUKh8zkXYiRQF");


#[program]
mod hello_anchor {
    use super::*;
    pub fn set_data(ctx: Context<SetData>, data: String) -> Result<()> {
        ctx.accounts.my_account.data = data;
        Ok(())
    }
}


#[account]
#[derive(Default)]
pub struct MyAccount {
    data: String
}


#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>
}