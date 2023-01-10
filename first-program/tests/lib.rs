// tokio  

use std::{str::FromStr, borrow::Borrow};

use helloworld::{process_instruction, GreetedCounter};
use solana_program::instruction::AccountMeta;
use solana_program_test::*;

use solana_sdk::{

    account::Account, 
    instruction::Instruction, 
    pubkey::Pubkey,
    signature::Signer, 
    transaction::Transaction
};
use borsh::{BorshDeserialize,BorshSerialize};

#[tokio::test] 
async fn test_helloworld() {

    //program id 
    // accounts 
    // instruction data 
    let program_id = Pubkey::from_str("FLJ6D7GrKsPPw7gZ57xUy1uEtwFs9ygZ6jMGqJVun15C").unwrap(); 

    let account = Pubkey::new_unique(); 

    // Program Test 
    let mut program_test = ProgramTest::new(
        "helloworld", 
        program_id,
        processor!(process_instruction)
    );

    program_test.add_account(account, Account{
        lamports:5, 
        data:vec![0_u8;4],
        owner:program_id,
        ..Account::default()

    });


    let (mut client, mut  payer, mut recblock_hash) = program_test.start().await; 

    let greeted_account = client.get_account(account).await.expect("get_account").expect("unable to get account ");

    assert_eq!(

            GreetedCounter::try_from_slice(&greeted_account.data.borrow()).unwrap().counter,
            0
    );

    let mut txn  = Transaction::new_with_payer(
            &[Instruction::new_with_bincode(program_id, &[0], vec![AccountMeta::new(account,false),AccountMeta::new(account,false),AccountMeta::new(account,false)])],

            Some(&payer.pubkey())

    );

    txn.sign(&[&payer], recblock_hash);
    client.process_transaction(txn).await.unwrap();
// BPFloader 
    let greeted_account = client.get_account(account).await.expect("get_account").expect("unable to get account ");


    assert_eq!(

        GreetedCounter::try_from_slice(&greeted_account.data.borrow()).unwrap().counter,
        1
);


    
}

