use std::ops::{Add,Sub,Mul,Neg};
use std::fmt;
#[derive(Debug,PartialEq,Copy,Clone)]
struct Complex
{
    real: f64,
    imaginar:f64,
}
impl Complex
{
    fn new<T: Into<f64>,U: Into<f64>>(real:T, imaginar:U)->Self
    {
        Complex
        {
            real: real.into(),
            imaginar: imaginar.into(),
        }
    }

    fn conjugate(&self)->Self
    {
        Complex
        {
            real : self.real,
            imaginar: -self.imaginar,
        }
    }
}
impl From<i32> for Complex
{
    fn from(valoare:i32)->Self
    {
        Complex
        {
            real: valoare as f64,
            imaginar: 0.0,
        }
    }
}
impl From<f64> for Complex
{
    fn from(valoare:f64)->Self
    {
        Complex
        {
            real: valoare,
            imaginar: 0.0,
        }
    }
}
impl<T> Add<T> for Complex
where 
    T:Into<Complex>,
{
    type Output = Complex;
    fn add(self,other:T)->Self::Output
    {
        let other = other.into();
        Complex
        {
            real: self.real + other.real,
            imaginar: self.imaginar+other.imaginar,
        }
    }
}
impl<T> Sub<T> for Complex
where 
    T:Into<Complex>,
{
    type Output = Complex;
    fn sub(self,other:T)->Self::Output
    {
        let other = other.into();
        Complex
        {
            real: self.real - other.real,
            imaginar: self.imaginar-other.imaginar,
        }
    }
}
impl<T> Mul<T> for Complex
where 
    T:Into<Complex>,
{
    type Output = Complex;
    fn mul(self,other:T)->Self::Output
    {
        let other = other.into();
        Complex
        {
            real: self.real*other.real - self.imaginar*other.imaginar,
            imaginar: self.real*other.imaginar + self.imaginar*other.real,
        }
    }
}
impl Neg for Complex
{
    type Output = Complex;
    fn neg(self)->Self::Output
    {
        Complex
        {
            real: -self.real,
            imaginar: -self.imaginar,
        }
    }
}
impl fmt::Display for Complex
{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result
    {
        if self.real == 0.0 && self.imaginar == 0.0
        {
            write!(f,"0")
        }
        else {
            if self.imaginar == 0.0
            {
                write!(f,"{}",self.real)
            }
            else {
                if self.real == 0.0
                {
                    write!(f,"{}i",self.imaginar)
                }
                else {
                    if self.imaginar < 0.0
                    {
                        write!(f,"{}{}i",self.real,self.imaginar)
                    }
                    else {
                        write!(f,"{}+{}i",self.real,self.imaginar)
                    }
                }
            }

        }
    }
}
fn eq_rel(x:f64,y:f64)->bool
{
    (x-y).abs() < 0.001
}
macro_rules! assert_eq_rel
{
    ($x:expr,$y:expr)=>
    {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x,y);
        assert!(r,"{} != {}",x,y);
    }
}
fn main() {
    let a = Complex::new(1.0, 2.0);
    println!("a = {}", a);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imaginar, 2);

    let b = Complex::new(2.0, 3);
    println!("b = {}", b);
    let c = a + b;
    println!("c = {}", c);
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imaginar, 5);

    let d = c - a;
    println!("d = {}", d);
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    println!("e = {}", e);
    assert_eq_rel!(e.imaginar, -7);

    let f = (a + b - d) * c;
    println!("f = {}", f);
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    println!("h = {}",h);
    let i = h - (h + 5) * 2.0;
    println!("i = {}", i);
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    println!("j = {}", j);
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imaginar, 0);

    println!("ok!");
}
