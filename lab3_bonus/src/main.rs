use thiserror::Error;

#[derive(Error,Debug)]
enum Errors
{
    #[error("Not an ASCII character!")]
    NotASCII,
    #[error("Not a digit!")]
    NotDigit,
    #[error("Not a base16 character!")]
    NotBase16Digit,
    #[error("Not a letter!")]
    NotALetter,
    #[error("Not a printable character!")]
    NotPrintable,
}
fn to_uppercase(ch:char)->Result<char,Errors>
{
    if (ch < 'a') || (ch > 'z' )
    {
        Err(Errors::NotALetter)
    }
    else {
        Ok((ch as u8 - 32) as char)
    }
}
fn to_lowercase(ch:char)->Result<char,Errors>
{
    if (ch < 'A') || (ch > 'Z')
    {
        Err(Errors::NotALetter)
    }
    else {
        Ok((ch as u8 + 32) as char)
    }
}
fn print_char(ch:char)->Result<char,Errors>
{
    if (ch as u8) <= 31 || (ch as u8) == 127
    {
        Err(Errors::NotPrintable)
    }
    else {
        Ok(ch)
    }
}
fn char_to_number(ch:char)->Result<u32,Errors>
{
    if (ch as u8) > 127
    {
        Err(Errors::NotASCII)
    }
    else {
        if (ch < '0') || (ch > '9')
        {
            Err(Errors::NotDigit)
        }
        else {
            Ok((ch as u8 - b'0') as u32)
        }
    }
}
fn char_to_number_hex(ch:char)->Result<u8,Errors>
{
    if (ch as u8) > 127
    {
        Err(Errors::NotASCII)
    }
    else {
        if (ch < '0' || ch > '9') && (ch < 'A' || ch > 'F')
        {
            Err(Errors::NotBase16Digit)
        }
        else {
            if ch >= '0' && ch <= '9'
            {
                Ok((ch as u8 - b'0') as u8)
            }
            else {
                Ok(ch as u8 - 'A' as u8 + 10)
            }
        }
    }
}
fn main() 
{
   let ch1 = 'y';
   match to_uppercase(ch1)
   {
        Ok(c)=>println!("to_uppercase: {c}"),
        Err(e)=>println!("Error: {}",e),
   }

   let ch2 = 'R';
   match to_lowercase(ch2)
   {
        Ok(c)=>println!("to_lowercase: {c}"),
        Err(e)=>println!("Error: {}",e),
   }

   let ch3 = '\n';
   match print_char(ch3)
   {
        Ok(c)=>println!("print_char: {c}"),
        Err(e)=>println!("Error: {}",e),
   }
   
   let ch4 = '9';
   match char_to_number(ch4)
   {
        Ok(c)=>println!("char_to_number: {c}"),
        Err(e)=>println!("Error: {}",e),
   }

   let ch5='G';
   match char_to_number_hex(ch5)
   {
        Ok(c)=>println!("char_to_number_hex: {c}"),
        Err(e)=>println!("Error: {}",e),
   }
}
