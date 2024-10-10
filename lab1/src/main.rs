fn primes(n:i32)->bool
{
    if n < 2 { return false; }
    let mut d = 2;
    while d < n 
    {
        if n%d == 0
        {
            return false;
        }
        d+=1;
    }
    return true;
}
fn coprime(mut a:i32,mut b:i32)->bool
{
    let mut r:i32;
    while b != 0
    {
        r = a % b;
        a = b;
        b = r;
    }
    if a == 1
    {
        return true;
    }
    return false;
}
fn bottles_ex3()
{
    let mut bottles = 99;
   loop {
       println!("{bottles} bottles of beer on the wall,");
       println!("{bottles} bottles of beer.");
       println!("Take one down, pass it around,");
       bottles-=1;
       println!("{bottles} bottles of beer on the wall.\n");

       if bottles == 1
       {
            println!("{bottles} bottle of beer on the wall,");
            println!("{bottles} bottle of beer.");
            println!("Take one down, pass it around,");
            println!("No bottles of beer on the wall.");
            break;
       }
   }
}
fn main() {

    //1.
    println!("Numerele prime de la 0 la 100 sunt:");
    let mut i = 0;
    while i < 101
    {
        let result=primes(i);
        if result == true
        {
            println!("{i}");
        }
        i+=1;
    }
    //2.
    let mut x = 2;
    while x < 100
    {
        let mut y = x+1;
        while y < 101
        {
            let result = coprime(x,y);
            if result == true { println!("{x} si {y} sunt coprime");}
            y+=1;
        }
        x+=1;
    }
    println!("\n");

    //3.
    bottles_ex3();
}
