use crate::state::game::*;
use anchor_lang::prelude::*;

pub fn initialize_game(ctx: Context<InitializeGame>, game_number: u64) -> Result<()> {
    ctx.accounts
        .game
        .initialize_game(ctx.accounts.initiator.key(), game_number)
}

#[derive(Accounts)]
#[instruction(game_number: u64)]
pub struct InitializeGame<'info> {
    #[account(init, payer = initiator, space = Game::MAXIMUM_SIZE + 8, seeds = [b"game", initiator.key().as_ref(), game_number.to_le_bytes().as_ref()], bump)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub initiator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
