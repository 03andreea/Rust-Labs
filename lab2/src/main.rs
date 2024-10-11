//EX1+EX2
fn add_chars_n(mut s:String,c:char,mut n:i32)->String
{
    while n > 0
    {
        s.push(c);
        n=n-1;
    }
    return s;
}
fn add_chars_n_ref(s:&mut String,c:char,mut n:i32)
{
    while n > 0
    {
        s.push(c);
        n=n-1;
    }
}
//EX3
fn add_space(s:&mut String,mut n:i32)
{
    while n > 0
    {
        s.push(' ');
        n-=1;
    }
}
fn add_str(s:&mut String,str:&str)
{
    s.push_str(str);
}
fn add_integer(s:&mut String,mut n:i32)
{
    let mut v=[0;32];
    let mut k = 0;
    while n > 0
    {
        v[k]=n%10;
        n=n/10;
        k+=1;
    }
    let mut i = k as i32 - 1;
    while i >= 0
    {
        let cif=(v[i as usize] as u8)+b'0';
        if (i%3 == 2) && (i != k as i32 -1)
        {
            s.push('_');
        }
        s.push(cif as char);
        i-=1;
    }
}
fn int_2_string(s:&mut String,mut n:i32)
{
    let mut v=[0;32];
    let mut k = 0;
    while n > 0
    {
        v[k]=n%10;
        n=n/10;
        k+=1;
    }
    let mut i = k as i32 - 1;
    while i >= 0
    {
        let cif=(v[i as usize] as u8) + b'0';
        s.push(cif as char);
        i-=1;
    }
}
fn add_float(s:&mut String,n:f32)
{
    let parte_intreaga= n as i32;
    int_2_string(s,parte_intreaga);
    s.push('.');
    let mut parte_fractionara= n - (parte_intreaga as f32);
    for _ in 0..3 {
        parte_fractionara*=10.0;
        let cifra= parte_fractionara as i32;
        s.push((cifra as u8 + b'0') as char);
        parte_fractionara-=cifra as f32;
    }
    
}
fn main() {

    //p1+p2
    let mut s1 = String::from("");
    let mut s2=String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        //APEL pentru exercitiul 1 (fara referinta) 
        s1 = add_chars_n(s1,c,26-i);
        //APEL pentru exercitiul 2 (cu referinta)
        add_chars_n_ref(&mut s2, c, 26 - i); 

        i += 1;
    }
    println!("Exercitiul 1:");
    println!("{s1}");
    println!("Exercitiul 2:");
    println!("{s2}");
    println!("\n");

    //EXERCITIUL 3
    let mut s:String = String::new();
    add_space(&mut s,41);
    add_str(&mut s,"I 💚");
    add_str(&mut s, "\n");
    add_space(&mut s,41);
    add_str(&mut s,"RUST.\n\n");
    add_space(&mut s,4);
    add_str(&mut s,"Most");
    add_space(&mut s,13);
    add_str(&mut s,"crate");
    add_space(&mut s,6);
    add_integer(&mut s,306437968);
    add_space(&mut s,11);
    add_str(&mut s,"and");
    add_space(&mut s,6);
    add_str(&mut s,"lastest");
    add_space(&mut s,9);
    add_str(&mut s,"is");
    add_str(&mut s,"\n");
    add_space(&mut s,9);
    add_str(&mut s,"downloaded");
    add_space(&mut s,8);
    add_str(&mut s,"has");
    add_space(&mut s,14);
    add_str(&mut s,"downloads");
    add_space(&mut s,6);
    add_str(&mut s,"the");
    add_space(&mut s,9);
    add_str(&mut s,"version");
    add_space(&mut s,5);
    add_float(&mut s,2.038);
    add_str(&mut s,".");
    println!("Exercitiul 3:");
    println!("{s}");
}
