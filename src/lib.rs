use byteorder::{ByteOrder, LittleEndian};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use std::mem;

use dingirsu_reflect::Dingirsu_Reflect;
use proc_macro;
struct Account_Sotrage
{
    //accounts:[AccountInfo;32]
}


//let g_PubKey:Pubkey;Pubkey::new(&[0; 32]);
//let g_Storage_Accounts:[Pubkey;32]=[Pubkey:: ;32];
// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    let mut pid=program_id;
    unsafe{

        let mut pointer :*mut u64= 0 as *mut u64;
    }

    //let a =solana_program::pubkey::Pubkey::new(&[0;32]);
//    msg!("Helloworld Rust program entrypoint");
//    
//    // Iterating accounts is safer then indexing
//    let accounts_iter = &mut accounts.iter();
//    
//    // Get the account to say hello to
//    let account:&AccountInfo = next_account_info(accounts_iter).unwrap();
//    //solana_sdk::account::create_account(sysvar: &S, lamports: u64)
//    //solana_sdk::account::AccountSharedData::from(account.owner);
//    let s=account.is_signer;
//    // The account must be owned by the program in order to modify its data
//    if account.owner != program_id {
//        msg!("Greeted account does not have the correct program id");
//        return Err(ProgramError::IncorrectProgramId);
//    }
//
//    // The data must be large enough to hold a u32 count
//    if account.try_data_len()? < mem::size_of::<u32>() {
//        msg!("Greeted account data length too small for u32");
//        return Err(ProgramError::InvalidAccountData);
//    }
//
//    // Increment and store the number of times the account has been greeted
//    let mut data = account.try_borrow_mut_data()?;
//    let mut num_greets = LittleEndian::read_u32(&data);
//    num_greets += 1;
//    LittleEndian::write_u32(&mut data[0..], num_greets);
//
    msg!("Hello!");
//

    let mut temp_str:Vec<u8>= Vec::new();
    let mut str_iter:std::slice::Iter<u8>=_instruction_data.iter();
    let mut s_item:Option<&u8>=None;
    loop{
        s_item= str_iter.next();
        if (s_item==None){
            break;
        }
        let s_u=s_item.unwrap();
        if (*s_u==0){
            break;
        }
        temp_str.push(s_u.clone());
        
    }    
    
    let mut func_name=String::from_utf8(temp_str ).unwrap();
    //let s_test="A";
    //msg!("{0};",s_test.len());
    //msg!("{0};",func_name.as_mut().len());
    let func=D_Processer::Reflect(func_name.as_mut());
    //msg!("{0}",func.is_some());
    if (func.is_some())
    {
        
        let func_u= func.unwrap();
        func_u(program_id,accounts,_instruction_data);
    }
    Ok(())
}


struct D_Processer {

}
Dingirsu_Reflect!{

    impl D_Processer {
        pub fn A(
            program_id: &Pubkey, // Public key of the account the hello world program was loaded into
            accounts: &[AccountInfo], // The account to say hello to
            _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
        ) -> ProgramResult {
            msg!("A");
            print!("A");
            Ok(())
        }
        pub fn B(
            program_id: &Pubkey, // Public key of the account the hello world program was loaded into
            accounts: &[AccountInfo], // The account to say hello to
            _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
        ) -> ProgramResult {
            msg!("B");
            print!("B");
            Ok(())
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
    #[test]
    fn it_works() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        LittleEndian::write_u32(&mut data, 0);
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
        let mut instruction_data: Vec<u8> = Vec::new();
        let mut s=String::from("A\0");
        let datas=s.as_bytes();
        for  i in datas
        {
            instruction_data.push(*i);
        }

        let accounts = vec![account];

        
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();


        assert_eq!(0,1);
    }
}


