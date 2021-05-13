# Solana_Reflect

## Rust
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


## JS
    The instruction data starts with a string wihch is a function name and ends with '\0'
    
        let t_data=Buffer.alloc(4);
        t_data[0]=65;
        t_data[1]=0;

        console.log('Saying hello to', prog.toBase58());
        const instruction = new TransactionInstruction({
          keys: [{pubkey: greetedPubkey, isSigner: false, isWritable: true}],
          programId:prog,
          data:t_data, // All instructions are hellos
        });
        let res= await sendAndConfirmTransaction(
          connection,
          new Transaction().add(instruction),
          [acc],
          {
            commitment: 'singleGossip',
            preflightCommitment: 'singleGossip',
          },
        );

## Program Log

        Program 54wzi12wkAq9vEtvxqVZBcgESm9aArrtzcReN7kJ8mwQ invoke [1]
        Hello!
        Reflect to function A
        Program 54wzi12wkAq9vEtvxqVZBcgESm9aArrtzcReN7kJ8mwQ consumed 966 of 200000 compute units
        Program 54wzi12wkAq9vEtvxqVZBcgESm9aArrtzcReN7kJ8mwQ success

