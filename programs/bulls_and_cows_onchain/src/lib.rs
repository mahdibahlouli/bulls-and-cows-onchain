use anchor_lang::prelude::*;
use solana_program::clock::Clock;

// defining our programid
declare_id!("28rwUTUkRqC9ZgqXMdR9GxWFuUo9PPyqhHwMALZLBbLU");

//defining a solana program module that contains all the instructions
#[program]
// we use pub so the functions and modules inside anchor bac can be used externally
pub mod anchor_bac {
    // to allow the use of all the components from the parent module without redeclaring them
    use super::*;
    use std::cmp::Ordering;
    // now the program initialisation is done ✅

    // making a function that initialize random numbers and records them to the context 
    //<()> this means that nothing is returned on success and Err is returned on the fail 
    pub fn initialize(ctx: Context<AccountContext>) -> Result<()> {
        // i need to import the guessing account from the context part 
        let guessing_account = &mut ctx.accounts.guessing_account;
        guessing_account.number = generate_random_number();
        Ok(())
    }
    // making a function that compares the entered guess by the user to the number field stored in
    // the data account and returning error messages from MyError  
    pub fn guess(ctx: Context<AccountContext>, number: u32) -> Result<()> {

        let guessing_account = &mut ctx.accounts.guessing_account;
        let target = guessing_account.number;

        match number.cmp(&target){
            Ordering::Less=>{return err!(MyError::NumberTooSmall)}
            Ordering::Greater=>{return err!(MyError::NumberTooLarge)}
            Ordering::Equal=>{return Ok(())}
        }


    }
}

fn generate_random_number() -> u32 {
    let clock = Clock::get().expect("Failed");
    let second = (clock.unix_timestamp % 10)  as u8;
    let result = (second + 1) as u32;
    result 
}

// creating a data account to store the guessing data 
#[account]
pub struct GuessingAccount{
    pub number: u32,
}

//derive transforms the struct into accounts in the blockchain 
#[derive(Accounts)]
pub struct AccountContext<'info>{
    #[account(
        // tells anchor to init the account if its not initialised yet 
        init_if_needed,
        // make space of 32 bytes which should be enough for GuessingAccount 
        space = 32, 
        payer = payer, 
        //generating PROGRAM DERIVED DATA 
        seeds = [b"guessing pda"],
        bump 
    )]
    // attach the GuessingAccount data account to the context and its alive as long as the
    // AccountContext struct is valid 
    pub guessing_account : Account<'info,GuessingAccount>,
    // making refferences for the payer account which is supposed to be mutable and the system progam and making sure they follow the longevity of the Account context 
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program : Program<'info, System>,
}

#[error_code]
pub enum MyError {
    #[msg("too small")]
    NumberTooSmall,
    #[msg("too large")]
    NumberTooLarge
}
