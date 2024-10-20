//ex 1
fn is_prime(x:u16)->bool
{
    if x < 2 {return false;}
    let mut d = 2;
    while d < x
    {
        if x % d == 0
        {
            return false;
        }
        d+=1;
    }
    return true;
}
fn next_prime(x:u16)->Option<u16>
{
    let mut y = x+1;
    while y < 65_535
    {
        if is_prime(y)
        {
            return Some(y);
        }
        y+=1;
    }
    None   
}
fn main_ex1() 
{
    
    let mut nr = 0;
    loop {
        match next_prime(nr)
        {
            Some(prime) => { println!("{}",prime); nr = prime;},
            None =>{ println!("Not an u16 number"); break;},
        }
    }
}

//ex2
fn checked_addition(a:u32,b:u32)->Result<u32,&'static str>
{
    if a  > 4_294_967_295 - b
    {
       return Err("Overflow at addition!");
    }
    Ok(a+b)
}
fn checked_multiplication(a:u32,b:u32)->Result<u32,&'static str>
{
    if (a != 0) && (b > 4_294_967_295/a)
    {
       return Err("Overflow at multiplication!");
    }
    Ok(a*b)
}
fn main_ex2()
{
    //exemplu Ok
    println!("---->Exemplu pentru Ok:");
    let a:u32=12;
    let b:u32=429304;
    let sum = checked_addition(a,b);
    match sum
    {
        Ok(rez)=>println!("Suma numerelor {a} si {b} este: {rez}"),
        Err(e)=>{ panic!("{e}");},
    }
    let prod=checked_multiplication(a,b);
    match prod
    {
        Ok(rez)=>println!("Produsul numerelor {a} si {b} este: {rez}"),
        Err(e)=> panic!("{e}"),
    }
    /* //exemplu cu panic - este comentat pt ca daca imi da panic nu mai continua executia celorlalte ex:)
    println!("---->Exemplu cu panic!");
    //exemplu panic
    let c:u32=4234752356;
    let d:u32=42930446;
    let sum2 = checked_addition(c,d);
    match sum2
    {
        Ok(rez)=>println!("Suma numerelor {c} si {d} este: {rez}"),
        Err(e)=> panic!("{e}"),
    }
    let prod2=checked_multiplication(c,d);
    match prod2
    {
        Ok(rez)=>println!("Produsul numerelor {c} si {d} este: {rez}"),
        Err(e)=> panic!("{e}"),
    }*/
}

//ex3
#[derive(Debug)]
enum Erori {
    DepasireAdunare,
    DepasireProdus,
}
fn checked_addition2(a:u32,b:u32)->Result<u32,Erori>
{
    if a  > 4_294_967_295 - b
    {
       Err(Erori::DepasireAdunare)
    }
    else {
        Ok(a+b)
    }
}
fn checked_multiplication2(a:u32,b:u32)->Result<u32,Erori>
{
    if (a != 0) && (b > 4_294_967_295/a)
    {
        Err(Erori::DepasireProdus)
    }
    else {
        Ok(a*b)
    }
}
fn check_operation(a:u32,b:u32)
{
    match checked_addition2(a,b)
    {
        Ok(suma) => println!("Succes: Suma nr {a} si {b} este {suma}"),
        Err(e) => println!("Error found: {:?}",e),
    }

    match checked_multiplication2(a,b)
    {
        Ok(prod) => println!("Succes: Produsul nr {a} si {b} este {prod}"),
        Err(e) => println!("Error found: {:?}",e),
    }
}
fn main_ex3()
{
    let a:u32=12;
    let b:u32=4293094504;
    check_operation(a,b);
}
 
//ex4
#[derive(Debug)]
enum Errors
{
    NotASCII,
    NotDigit,
    NotBase16Digit,
    NotALetter,
    NotPrintable,
}
fn to_uppercase(ch:char)->Result<char,Errors>
{
    if (ch < 'a') || (ch > 'z') 
    {
        Err(Errors::NotALetter)
    }
    else {
        Ok((ch as u8-32) as char)
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
    if ((ch as u8) <= 31) || ((ch as u8) == 127)
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
            Ok(((ch as u8)-b'0') as u32)
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
fn print_error(e:Errors)
{
    match e
    {
        Errors::NotASCII=> println!("Error found: Not an ASCII character!"),
        Errors::NotDigit=> println!("Error found: Not a digit!"),
        Errors::NotBase16Digit=> println!("Error found: Not a base16 character!"),
        Errors::NotALetter=> println!("Error found: Not a letter!"),
        Errors::NotPrintable=> println!("Error found: Not a printable character!"),
    }
}
fn main_ex4()
{
    //to_uppercase
    let ch1 = 'y';
    match to_uppercase(ch1)
    {
        Ok(c) => println!("to_uppercase: {c}"),
        Err(e) => print_error(e),
    }
    //to_lowercase
    let ch2 = 'R';
    match to_lowercase(ch2)
    {
        Ok(c)=> println!("to_lowercase: {c}"),
        Err(e)=> print_error(e),
    }
    //print_char
    let ch3 = '\n';
    match print_char(ch3)
    {
        Ok(c)=> println!("print_char: {c}"),
        Err(e)=> print_error(e),
    }
    //char_to_number
    let ch4 = '9';
    match char_to_number(ch4)
    {
        Ok(c)=>println!("char_to_number: {c}"),
        Err(e)=> print_error(e),
    }
    //char_to_number_hex
    let ch5 = 'A';
    match char_to_number_hex(ch5)
    {
        Ok(c)=> println!("char_to_number_hex: {c}"),
        Err(e)=> print_error(e),
    }
}

//ex5 
#[derive(Debug)]
enum EroriOperatii
{
    NotADigit,
    NotAnOperator,
    WrongOperator,
    DivideBy0,
}
fn adunare(a:char,op:char,b:char)->Result<u8,EroriOperatii>
{
    if op != '+' && op != '-' && op != '*' && op != '/' && op != '%'
    {
        Err(EroriOperatii::NotAnOperator)
    }
    else {
            if op != '+'
            {
                Err(EroriOperatii::WrongOperator)
            }
            else {
                    if (a < '0' || a > '9') || (b < '0' || b > '9')
                    {
                        Err(EroriOperatii::NotADigit)
                    }
                    else {
                        Ok(((a as u8 - b'0')+(b as u8 - b'0')) as u8)
                    }
            }
    }

}
fn scadere(a:char,op:char,b:char)->Result<u8,EroriOperatii>
{
    if op != '+' && op != '-' && op != '*' && op != '/' && op != '%'
    {
        Err(EroriOperatii::NotAnOperator)
    }
    else {
            if op != '-'
            {
                Err(EroriOperatii::WrongOperator)
            }
            else {
                if (a < '0' || a > '9') || (b < '0' || b > '9')
                {
                    Err(EroriOperatii::NotADigit)
                }
                else {
                    Ok(((a as u8 - b'0')-(b as u8 - b'0')) as u8)
                }
            }
    }
}
fn inmultire(a:char,op:char,b:char)->Result<u8,EroriOperatii>
{
    if op != '+' && op != '-' && op != '*' && op != '/' && op != '%'
    {
        Err(EroriOperatii::NotAnOperator)
    }
    else {
            if op != '*'
            {
                Err(EroriOperatii::WrongOperator)
            }
            else {
                if (a < '0' || a > '9') || (b < '0' || b > '9')
                {
                    Err(EroriOperatii::NotADigit)
                }
                else {
                    Ok(((a as u8 - b'0')*(b as u8 - b'0')) as u8)
                }
            }
    }
}
fn cat(a:char,op:char,b:char)->Result<u8,EroriOperatii>
{
    if op != '+' && op != '-' && op != '*' && op != '/' && op != '%'
    {
        Err(EroriOperatii::NotAnOperator)
    }
    else {
            if op != '/'
            {
                Err(EroriOperatii::WrongOperator)
            }
            else {
                if (a < '0' || a > '9') || (b < '0' || b > '9')
                {
                    Err(EroriOperatii::NotADigit)
                }
                else {
                        if b as u8 -b'0' == 0
                        {
                            Err(EroriOperatii::DivideBy0)
                        }
                        else {
                            Ok(((a as u8 - b'0')/(b as u8 - b'0')) as u8)
                        }
                }
            }
    }
}
fn rest(a:char,op:char,b:char)->Result<u8,EroriOperatii>
{
    if op != '+' && op != '-' && op != '*' && op != '/' && op != '%'
    {
        Err(EroriOperatii::NotAnOperator)
    }
    else {
            if op != '%'
            {
                Err(EroriOperatii::WrongOperator)
            }
            else {
                if (a < '0' || a > '9') || (b < '0' || b > '9')
                {
                    Err(EroriOperatii::NotADigit)
                }
                else {
                        if b as u8 -b'0' == 0
                        {
                            Err(EroriOperatii::DivideBy0)
                        }
                        else {
                            Ok(((a as u8 - b'0')%(b as u8 - b'0')) as u8)
                        }
                }
            }
    }
}
fn afiseaza_eroare(e:EroriOperatii)
{
    match e
    {
        EroriOperatii::NotADigit=> println!("Error found: a or b is not a digit!"),
        EroriOperatii::NotAnOperator=> println!("Error found: the operator does not exist!"),
        EroriOperatii::WrongOperator=> println!("Error found: the wrong operator was used!"),
        EroriOperatii::DivideBy0=>println!("Error found: divide by 0!"),
    }
}
fn main_ex5()
{
    let a1 = '7';
    let b1 = '6';
    let op1 = '=';
    match adunare(a1,op1,b1)
    {
        Ok(sum)=>println!("Succes: Suma este {sum}"),
        Err(e)=> afiseaza_eroare(e),
    }

    let a2 = '8';
    let b2 = '2';
    let op2 = '-';
    match scadere(a2,op2,b2)
    {
        Ok(dif)=>println!("Succes: Diferenta este {dif}"),
        Err(e)=> afiseaza_eroare(e),
    }

    let a3 = '2';
    let b3 = '5';
    let op3 = '*';
    match inmultire(a3,op3,b3)
    {
        Ok(mul)=>println!("Succes: Produsul este {mul}"),
        Err(e)=> afiseaza_eroare(e),
    }

    let a4 = '9';
    let b4 = '5';
    let op4 = '/';
    match cat(a4,op4,b4)
    {
        Ok(cat)=>println!("Succes: Catul este {cat}"),
        Err(e)=> afiseaza_eroare(e),
    }

    let a5 = '9';
    let b5 = '5';
    let op5 = '/';
    match rest(a5,op5,b5)
    {
        Ok(rest)=>println!("Succes: Catul este {rest}"),
        Err(e)=> afiseaza_eroare(e),
    }

}
fn main()
{
    println!("---------------------------ex1------------------------");
    main_ex1();
    println!("---------------------------ex2------------------------");
    main_ex2();
    println!("---------------------------ex3------------------------");
    main_ex3();
    println!("---------------------------ex4------------------------");
    main_ex4();
    println!("---------------------------ex5------------------------");
    main_ex5();
    
}
