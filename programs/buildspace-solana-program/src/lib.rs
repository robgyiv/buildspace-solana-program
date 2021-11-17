use anchor_lang::prelude::*;

declare_id!("GLDmKb1CRS3fS58G658Nd8kfkuyAkQRX9WP9Y68RVf5F");

#[program]
pub mod buildspace_solana_program {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // Get a reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // Initialise total_gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        // Get a reference to the account and increment total_gifs
        let base_account = &mut ctx.accounts.base_account;

        // Build the struct
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *base_account.to_account_info().key,
            votes: 0,
        };

        // Add it to the gif_list vector
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn upvote_item(ctx: Context<UpvoteItem>, gif_index: i32) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        // TODO: Don't cast to usize here? 
        base_account.gif_list[gif_index as usize].votes += 1;
        Ok(())
    }
}

// Attach certain variables to the StartStuffOff context
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Specify what data you want in the AddGif context
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// Create a custom struct to work with
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes: i32,
}

// Tell Solana what we want to store on the account
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a vector of type ItemStruct to the account
    pub gif_list: Vec<ItemStruct>,
}

// Specify what data you want in the AddGif context
#[derive(Accounts)]
pub struct UpvoteItem<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}
