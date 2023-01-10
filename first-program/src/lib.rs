// 1- account 
// AccountInfo 
// Account -> Native Programs/ Accounts 
/*
10MB 
data: 
smartcontracts/programs -> executable 
*/

//2 - programs 
// stateless -> State/data 
// logic 
// data:ByteCode / bpf 
// 
// All programss on cluster are owned by Native program -> BPFLOADER 
// bpf -> bytecode format ->  c++/RUst/C -> bpf bytecode 
// entrypoint! 
// 1- address/pubkey of your program -> program_id
// 2- slice/array of Accounts to which above program id will interact with | data 
// 3- instruction_data -> 
// 4- Processes 

// txn -> sender | Reciever 
//[A1, A2] => Sender | Reciever 

// TXN 
    // Instruction
    // Solana runtime  -> BPFLODAER -> entrypoint! 
    // accounts / program_id/ instruction 
    // Instruction -> accounts/program_id/instruction_data 
    
 // Instruction
    // Client -> txn -> Order    

use borsh::{BorshDeserialize,BorshSerialize}; 
use solana_program::{

    account_info::{next_account_info,AccountInfo },
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
    clock::Epoch
};

/*
    onchain -> Counter [how many times the account is interacted to say hello ]
    
    // counter -> how many times this program_id got called 
    // counter: u32 
    struct GreetedCounter {
        counter:u32
    }

    entrypoint! 
    
    three arguments
        1- program_id
        2- accounts[]
        3- instrcution_data []


    
*/

#[derive(Debug,BorshDeserialize,BorshSerialize)]
pub struct  GreetedCounter{
    pub counter:u32
}



entrypoint!(process_instruction) ; 

pub fn process_instruction(

    program_id:&Pubkey, // program_id of this smart contract / multiple  -> instruction
    accounts:&[AccountInfo],
    _instruction_data:&[u8] // client serialized -> deser

) -> ProgramResult {


    //stateless -> Nodata  
    // accounts -> data

    // extract accounts 

    let mut accounts_iter = &mut  accounts.iter(); 

    // one by one 
    // Order 
  // Sender Reciver 
   //Vault  [owner, caretaker, authority]
  let account = next_account_info(accounts_iter)?; // 0
//   let sender = next_account_info(accounts_iter)?; // 1
//   let reciever = next_account_info(accounts_iter)?; // 2



    // only the program_id can modify this program 

    if account.owner != program_id {
        msg!(" owner should be program id ");
        return Err(ProgramError::IncorrectProgramId);
    }

    // extract data from account 
    // to store data in account
 
    // deserailzed the account data 
     let mut greeted_counter = GreetedCounter::try_from_slice(&account.data.borrow())?;
     // modified the state
    greeted_counter.counter+=1; 

    // serialze it again 

    greeted_counter.serialize(&mut &mut  account.data.borrow_mut()[..])?;
     
    msg!(" counter update ");






  // unwrap -> panic
  // ? return Err



    Ok(())


}


// mainnet  -> beta
// devnet -> 
// testnet 
// localnet 

#[cfg(test)]
mod tests{


    use super::* ;


    #[test]
    fn test_helloworld() {
    
    
        // program_id
        // accounts 
        // instruction_data
        let program_id = Pubkey::default();
    
        let key = Pubkey::default(); // 
    
        let mut data = vec![0_u8;4];
    
        let owner = Pubkey::default();
        let mut lamports :u64= 0; 
        let account = AccountInfo::new(
    
            &key, 
            false,
            true, 
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default()
        
    
        );
    
     let mut instruction_data:Vec<u8> = vec![]    ;
    
     let accounts  = vec![account];
    // Refcell
    
      assert_eq!(
    
        GreetedCounter::try_from_slice(&accounts[0].data.borrow()).unwrap().counter,
        0
    
      );
    
    
      process_instruction(&program_id, &accounts, &instruction_data);
    
      assert_eq!(
    
        GreetedCounter::try_from_slice(&accounts[0].data.borrow()).unwrap().counter,
        1
    
      );
    
      process_instruction(&program_id, &accounts, &instruction_data);
    
        
      assert_eq!(
    
        GreetedCounter::try_from_slice(&accounts[0].data.borrow()).unwrap().counter,
        2
    
      );
    
    
    }

}



