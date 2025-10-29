use anchor_lang::prelude::*;

declare_id!("4N8f8zmwb3cuT8wMYAbEqRw47UxrLfwk3HXiCmJMwmvu");

#[program]
pub mod hellosol {
    use super::*;

    pub fn handle(
        _ctx: Context<HandleCtx>,
        origin: u32,
        sender: [u8; 32],
        message: Vec<u8>,
    ) -> Result<()> {
        msg!("📨 Hyperlane message received!");
        msg!("From domain: {}", origin);
        msg!("Sender: {:?}", sender);
        msg!("Message: {}", String::from_utf8_lossy(&message));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct HandleCtx {}
