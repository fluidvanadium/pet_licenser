fn main() {
    println!("{:?}", CompletePet::Lizard);
}

#[derive(Debug)]
pub enum CompletePet {
    Cat,
    Dog,
    Fish,
    Lizard,
    Parrot,
}
