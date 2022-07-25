use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// import the instruction.rs
pub mod instruction;
use crate::instruction::HelloInstruction;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], 
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Send the instrcion_data obtained from client to the unpack function
    // to decode data to HelloInstruction enum
    let instruction = HelloInstruction::unpack(instruction_data)?;

    // Iterating accounts is safer than indexing
    // even though accounts is only borrowing or referecing an array with the
    // iter() function we are asking for a mutable account element of the accounts
    // array. So in Rust we are allowed to ask for mutable reference to a variable
    // even though accounts array was just an immutable refernce
    // iter() function creates an iterator over the &accounts array
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    // using the iterator obtain the accountInfo struct of the next account 
    // Since this is the first time we are calling the next_account_info() on
    // accounts_iter this will be the first element of the accounts and we will
    // get the account_info of that first account
    // this variable should have been called account_info instead as that is what
    // we are getting back
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    // de-serialize using the try_from_slice() function the reference to [u8] 
    // in the account.data
    // we get an instance of the struct GreetingAccount. we save it as a mutable
    // variable to change the field counter of the struct's instance
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;

    // instruction is an HelloInstruction enum already desctructured from the Result
    // Check what the instruction value unpacked to. Depending on the
    // enum variant we do the corresponding action of incrementing or decrementing
    // or setting the value
    match instruction {
        HelloInstruction::Increment => greeting_account.counter += 1,
        HelloInstruction::Decrement => greeting_account.counter -= 1,
        HelloInstruction::Set(value) => greeting_account.counter = value,
    }


    
    // storing the data as bytes by serializing it
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
