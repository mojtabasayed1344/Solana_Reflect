# Solana_Reflect

this code let developers generate function id and match function name with function automatically.

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
    
    let mut instruction_data: Vec<u8> = Vec::new();
    let mut s=String::from("A\0");
    let datas=s.as_bytes();
    for  i in datas
    {
        instruction_data.push(*i);
    }
