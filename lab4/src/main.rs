//p1

use std::{io,fs};
fn main_ex1() ->Result<(), io::Error>{

    let s = fs::read_to_string("D:/teo/ANUL 2/RUST/lab4/input.txt")?;
    let mut max_chars=0;
    let mut max_bytes=0;
    let mut line_bytes=String::new();
    let mut line_chars=String::new();
    for line in s.lines()
    {
        let mut chars=0;
        for _ in line.chars()
        {
            chars+=1;
        }
        if chars > max_chars
        {
            max_chars=chars;
            line_chars = line.to_string();
        }

        let mut bytes = 0;
        for _ in line.bytes()
        {
            bytes+=1;
        }
        if bytes > max_bytes 
        {
            max_bytes = bytes;
            line_bytes=line.to_string();
        }
    }
    println!("The longest line considering the number of bytes:   {line_bytes}");
    println!("The longest line considering the number of characters:    {line_chars}");
    Ok(())
}

//p2
fn main_ex2() -> Result<(),io::Error>
{
    let s =  fs::read_to_string("D:/teo/ANUL 2/RUST/lab4/input2.txt")?;
    let mut sir_final = String::new();
    for c in s.chars()
    {
        if c >= 'A' && c <= 'Z'
        {
            if ((c as u8) + 13) as char <= 'Z'
            {
                let ch=((c as u8)+13) as char;
                sir_final.push(ch);
            }
            else {
                let ch2=(((c as u8)+13) -26) as char;
                sir_final.push(ch2);
            }
        }
        else {
            if c >= 'a' && c <= 'z'
            {
                if ((c as u8) + 13) as char <= 'z'
                {
                    let ch3 = ((c as u8)+13)as char;
                    sir_final.push(ch3);
                }
                else {
                    let ch4 = (((c as u8)+13)-26) as char;
                    sir_final.push(ch4);
                }
            }
            else {
                if !c.is_ascii()
                {
                    eprintln!("Eroare: Non-Ascii character encountered.");
                    return Ok(());
                }
            }
        }
    }
    fs::write("D:/teo/ANUL 2/RUST/lab4/output.txt",sir_final)?;
    Ok(())
}

//p3
fn main_ex3()->Result<(),io::Error>
{
    let s = fs::read_to_string("D:/teo/ANUL 2/RUST/lab4/input3.txt")?;
    let mut result=String::new();
    for v in s.split(" ")
    {
         match v {
            "pt"=> result.push_str("pentru"),
            "ptr"=> result.push_str("pentru"),
            "Pt"=> result.push_str("Pentru"),
            "Ptr"=> result.push_str("Pentru"),
            "dl"=> result.push_str("domnul"),
            "Dl"=> result.push_str("Domnul"),
            "dna"=> result.push_str("doamna"),
            "Dna"=>result.push_str("Doamna"),
            _=> result.push_str(v),
        } 
        result.push(' ');
    } 
    println!("{result}");
    Ok(())
}

fn main_ex4() -> Result<(), io::Error> {
    let s = fs::read_to_string("D:/teo/ANUL 2/RUST/lab4/hosts_sample.txt")?;

    for line in s.lines() 
    {
        if !line.starts_with("#") && !line.is_empty()
        {
            let mut cuvinte = 0;
            let mut sir_nou:Vec<String> = Vec::new();
            for cuv in line.split_whitespace().collect::<Vec<&str>>()
            {
                if cuvinte < 2
                {
                    sir_nou.push(cuv.to_string());
                    cuvinte=cuvinte+1;
                }
            }
            println!("{} => {}",sir_nou[1],sir_nou[0]);

        }
    }

    Ok(())
}

fn main()->Result<(),io::Error>
{
    println!("----------------------------------ex1-----------------------------------");
    main_ex1()?;
    println!("----------------------------------ex2-----------------------------------");
    main_ex2()?;
    println!("Rezultatul apare in fisierul de output.txt!");
    println!("----------------------------------ex3-----------------------------------");
    main_ex3()?;
    println!("----------------------------------ex4-----------------------------------");
    main_ex4()?;
    Ok(())
}



   
