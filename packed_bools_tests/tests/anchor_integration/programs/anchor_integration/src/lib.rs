use anchor_lang::prelude::*;
use packed_bools::PackedBooleans;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_integration {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        // Get mut ref to user_account
        let ref mut user_account = ctx.accounts.user_account;

        // Set flags
        user_account.set_admin(false);
        user_account.set_tos_agree(true);
        user_account.set_og(true);

        // Check values
        assert_eq!(user_account.account_flags, 2_u8.pow(1) + 2_u8.pow(2));
        assert_eq!(user_account.get_admin(), false);
        assert_eq!(user_account.get_tos_agree(), true);
        assert_eq!(user_account.get_og(), true);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(
        init,
        payer = user,
        space = 9,
        seeds = [
            "my_account_seeds".as_bytes()
        ],
        bump,
    )]
    pub user_account: Account<'info, MyAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}


#[account]
#[derive(PackedBooleans)]
pub struct MyAccount {
    #[pack_bools(admin, tos_agree, og)]
    pub account_flags: u8,
}
