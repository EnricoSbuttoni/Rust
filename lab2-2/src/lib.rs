pub mod solution{
use std::{cmp::Ordering, ops::{Add, AddAssign}};
use std::fmt::Debug;

#[derive(Debug)]
pub struct ComplexNumber{
    real:f64,
    imag:f64,
}
impl ComplexNumber{
    pub fn new(a:f64, b:f64)->Self{
       ComplexNumber{real:a,imag:b}
    }
    pub fn  real(&self)->f64{
        self.real
    }
    pub fn  imag(&self)->f64{
        self.imag
    }
    pub fn from_real(a:f64)->Self{
        ComplexNumber{real:a,imag:0.0}
    }
    pub fn to_tuple(&self)->(f64,f64){
        (self.real,self.imag)
    }
}
impl Add<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, rhs: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }}
impl Add<f64> for ComplexNumber {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        ComplexNumber {
            real: self.real + other,
            imag: self.imag,
        }
    }

}
impl Add<&ComplexNumber> for ComplexNumber {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        ComplexNumber {
            real: self.real + other.real(),
            imag: self.imag + other.imag(),
        }
    }

}
impl Add<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real() + other.real(),
            imag: self.imag() + other.imag(),
        }
    }

}
impl AddAssign for ComplexNumber{

    fn add_assign(&mut self, rhs: Self) {
        self.real=self.real+rhs.real;
        self.imag=self.imag+rhs.imag;
    }
}
impl Clone for ComplexNumber{
    fn clone(&self) -> Self {
        Self { real: self.real.clone(), imag: self.imag.clone() }
    }
}
impl Copy for ComplexNumber{

}
impl Default for ComplexNumber{
    fn default() -> Self {
        Self { real: 0.0, imag: 0.0 }
    }
}
impl PartialEq for ComplexNumber{
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }

}


impl Eq for ComplexNumber{}
impl PartialOrd for ComplexNumber{

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mod1= self.real*self.real+self.imag+self.imag;
        let mod2= other.real*other.real+other.imag+other.imag;
        match mod1.partial_cmp(&mod2) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        mod1.partial_cmp(&mod2)
        }
}
impl Ord for ComplexNumber{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mod1= self.real*self.real+self.imag+self.imag;
        let mod2= other.real*other.real+other.imag+other.imag;
        if mod1 < mod2 {
            Ordering::Less
        } else if mod1 > mod2 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
        
    }
    
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }
    
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }
    
    
    }

   impl AsRef<f64> for ComplexNumber{
    fn as_ref(&self) -> &f64 {
        return &self.real;
    }
   }
   impl AsMut <f64> for ComplexNumber{
    fn as_mut(&mut self) -> &mut f64 {
        return & mut self.real;
    }
   }
}

