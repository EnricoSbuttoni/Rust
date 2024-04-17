pub mod solution{
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

}
}