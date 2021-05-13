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


    msg!("Reflect for Solana");

    let mut index=0;
    let mut temp_str:Vec<u8>= Vec::new();
    let mut str_iter:std::slice::Iter<u8>=_instruction_data.iter();
    let mut s_item:Option<&u8>=None;
    loop{
        s_item= str_iter.next();
        if (s_item==None){
            break;
        }
        let s_u=s_item.unwrap();
        index+=1;
        if (*s_u==0){
            break;
        }
        temp_str.push(s_u.clone());
        
    }    
    
    let mut func_name=String::from_utf8(temp_str ).unwrap();

    let func=D_Processer::Reflect(func_name.as_mut());

    if (func.is_some())
    {        
        let func_u= func.unwrap();
        func_u(program_id,accounts,&_instruction_data[index..]);
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
            msg!("Reflect to function A");
            print!("A");
            Ok(())
        }
        pub fn B(
            program_id: &Pubkey, // Public key of the account the hello world program was loaded into
            accounts: &[AccountInfo], // The account to say hello to
            _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
        ) -> ProgramResult {
            msg!("Reflect to function B");
            print!("B");
            Ok(())
            
        }
    }
    
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use solana_program::clock::Epoch;
//    #[test]
//    fn it_works() {
//        let program_id = Pubkey::default();
//        let key = Pubkey::default();
//        let mut lamports = 0;
//        let mut data = vec![0; mem::size_of::<u32>()];
//        LittleEndian::write_u32(&mut data, 0);
//        let owner = Pubkey::default();
//        let account = AccountInfo::new(
//            &key,
//            false,
//            true,
//            &mut lamports,
//            &mut data,
//            &owner,
//            false,
//            Epoch::default(),
//        );
//        let mut instruction_data: Vec<u8> = Vec::new();
//        let mut s=String::from("A\0");
//        let datas=s.as_bytes();
//        for  i in datas
//        {
//            instruction_data.push(*i);
//        }
//
//        let accounts = vec![account];
//
//        
//        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//
//
//        assert_eq!(0,1);
//    }
//}


