use anchor_lang::prelude::*;

declare_id!("4N8f8zmwb3cuT8wMYAbEqRw47UxrLfwk3HXiCmJMwmvu");

#[program]
pub mod hellosol {
    use super::*;

    pub fn receive_message(ctx: Context<Initialize>, message: Vec<u8>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Received: {:?}", String::from_utf8(message));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
