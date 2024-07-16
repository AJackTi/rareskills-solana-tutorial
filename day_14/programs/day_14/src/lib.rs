use anchor_lang::prelude::*;

declare_id!("CmU3PtiTkLQPZmpv4nhqMcHcPbS7YyxbYLmvnBxQoShs");

const OWNER: &str = "AzuiWapU4pttFt7qQLHaiQMcuhzVb2mwEok5LRWgZJZx";
const OWNERS: [&str; 2] = [
    "AzuiWapU4pttFt7qQLHaiQMcuhzVb2mwEok5LRWgZJZx",
    "ErsJfNcJMM41r952AFGgpeVyoCLQKHKqRGsY9W3t1otA",
];

#[program]
pub mod day_14 {
    use super::*;

    #[access_control(check(&ctx))]
    pub fn initialize(ctx: Context<OnlyOwner>) -> Result<()> {
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;

        msg!("The signer1: {:?}", *the_signer1.key);
        msg!("The signer2: {:?}", *the_signer2.key);

        msg!("Holla, I'm the owner.");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    // Check if signer === owner
    // require_keys_eq!(
    //     ctx.accounts.signer_account.key(),
    //     OWNER.parse::<Pubkey>().unwrap(),
    //     OnlyOwnerError::NotOwner
    // );

    let signer_key = ctx.accounts.signer_account.key();
    let owner_keys: Vec<Pubkey> = OWNERS.iter()
        .map(|owner| owner.parse::<Pubkey>().unwrap())
        .collect();

    if !owner_keys.contains(&signer_key) {
        return Err(OnlyOwnerError::NotOwner.into());
    }

    Ok(())
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
}

// An enum for custom error codes
#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}
