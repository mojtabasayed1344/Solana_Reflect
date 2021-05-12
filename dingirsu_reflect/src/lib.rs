extern crate proc_macro;
//extern crate syn;
//extern crate quote;

use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro]
pub fn Dingirsu_Reflect(input: TokenStream) -> TokenStream {
    
    let mut original_code =input.to_string();
    original_code=original_code.replace("\n", " ");
    let mut code:Vec<&str> =original_code.splitn(original_code.len()," ").collect();
    let a=code.len();
    let mut fn_list:Vec<&str>=Vec::new();

    for index in 0..code.len()
    {
        let item=code[index];
        
        if ((item=="fn" )  && index+1< code.len())
        {
            
            let t_fn_name:Vec<&str>=code[index+1].split("(").collect();
            fn_list.insert(0, Get_FunctionName(index,code.clone()).unwrap());
            
        }

        
    }
    
    for item in &fn_list
    {
        println!("{0}",item)
    }
    
    let mut inject_str:String=String::new();
    inject_str+="pub fn Reflect(name :&str)-> Option< fn(&solana_program::pubkey::Pubkey,&[solana_program::account_info::AccountInfo],&[u8])->solana_program::entrypoint::ProgramResult  >
    {
        match name
        {
    ";
    
    for item in &fn_list
    {
        
        let mut t_str:String=format!( "\"{0}\"=>{{
            
            return Some(Self::{1});
        }}
        ",*item,*item);
        inject_str+=t_str.as_mut();
    }


    inject_str+="
        _=>{
            return None;
            }
        }
    }
    
    ";
    let mut inject_str_mut=inject_str.as_mut();
    for index in 0..code.len()
    {
        let item=code[index];
        if (item=="impl" && index+3< code.len())
        {
            code.insert(index+3, inject_str_mut);
            break;
        }

        
    }
    let mut res_str=String ::new();
    for index in 0..code.len()
    {
        res_str+=code[index];
        res_str+=" ";
    }

    println!("{0}",res_str);

    let  res_ts:TokenStream =TokenStream::from_str(res_str.as_mut()).unwrap();
    return res_ts;
}


    fn Get_FunctionName(index:usize,code:Vec<&str>)-> Option<&str>
    {
        let mut i=index;
        if(code[index]=="fn" )
        {
            
        loop {
            i=i+1;
            let mut t_str:&str=code[i];
            if(  t_str.len()>0 && !(t_str.starts_with(" "))  )
            {
                let t_fn_name:Vec<&str>=code[i].split("(").collect();
                if(t_fn_name.len()>0)
                {
                    let t=t_fn_name[0];
                    return Some(t);
                }
            }
            
        }
    }
    
    return None;
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



