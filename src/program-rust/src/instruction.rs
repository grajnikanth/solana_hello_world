//src/program-rust/src/instruction.rs
// customizing Hello world contract

use solana_program::{program_error::ProgramError};
use std::convert::TryInto;

// The enum below will be used by the client to send us specific instruction to be
// executed in the smart contract
// Increment will increase counter by 1
// Decrement will decrease counter by 1
// Set will set the value of the counter to the u32 sent by client
// Debug macro to print out the enum value
#[derive(Debug)]
pub enum HelloInstruction {
    Increment,
    Decrement,
    Set(u32)
}


impl HelloInstruction {

    // implement a unpack function on this enum to take the client buffer and
    // decode it to the enum above
    // unpack will return a Self i.e, a HelloInstruction enum
    // If error, then we will return a solana defined ProgramError
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // split_first() function on &[u8] gives back an Option enum with value suchh that
        // first element of u8 is returned. If there is a problem None will be returned
        // We take the None and convert it to a Result error using ok_or() function.
        // this gives a result so if successful we will obtain the value by using ?
        // if there is an error the ? will propagate the error as a return to this 
        // function
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        // use match to conver the tag number to enum of HelloInstruction
        match tag {
            // Ok(T) is the first field of the std library Result<T,E>
            // enum Result<T, E> {
            //     Ok(T),
            //     Err(E),
            //  }
            // so below we are returning a Result<T> by wrapping enum with Ok()
            0 => return Ok(HelloInstruction::Increment),
            1 => return Ok(HelloInstruction::Decrement),
            2 => {
                // rest contains the rest of four elements of the &[u8]
                if rest.len() !=4 {
                    // note Err is the enum field of Result. See above
                    return Err(ProgramError::InvalidInstructionData);
                }
                // convert rest array slice to a fixed size array using
                // try_into() function (trait). We just have to tell Rust what type
                // of array this will be.
                // We do not care about the type of error if it erros out so we put
                // _ for the Err field
                // try_into() converts self in this case rest[..4] into a Type T
                // we specified which is the [u8: 4] - array of 4 u8's
                let val: Result<[u8; 4], _> = rest[..4].try_into();
                match val {
                    Ok(value) => return Ok(HelloInstruction::Set(u32::from_le_bytes(value))),
                    _ => return Err(ProgramError::InvalidInstructionData)
                }
            },
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}