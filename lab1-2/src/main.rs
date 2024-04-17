
struct Node {
    name: String,
    size: u32,
    count: u32,
    }
impl Node {
    pub fn new(name: String) -> Node {
    Node {name, size:0, count:0}
    }
    pub fn size(mut self, size:u32)-> Self{
        self.size=size;
        self
    }
    pub fn count(mut self, count:u32)-> Self{
        self.count=count;
        self
    }
    
    pub fn to_string(&self) -> String{
        let t =format!("Name: {}, Size: {}, Count: {}", self.name, self.size, self.count);
        t
    }
    }
    
fn main() {

    let prova=  Node::new(String::from("Nodo1")).size(19).count(4);


    println!("{}", prova.to_string());
}
