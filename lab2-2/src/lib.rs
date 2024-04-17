mod solution{
struct ComplexNumber{
    real:i32,
    imag:i32,
}
impl ComplexNumber{
    fn new(& mut self, a:i32, b:i32){
        self.real=a;
        self.imag=b;
    }
}
}