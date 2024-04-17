trait T1{
    fn returns_num()->i32;
    fn returns_self ()->Self;
}

struct TipodiEsempio;
impl T1 for TipodiEsempio{
    fn returns_num->i32{3};
    fn returns_self->Self{TipodiEsempio};
}

fn main() {
    println!("Hello, world!");
}
