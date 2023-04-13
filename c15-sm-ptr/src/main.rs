struct Pisa {
    hole_size: u8,
}

impl Pisa {
    fn stretch(&mut self) {
        println!("Auch!");
        self.hole_size += 1;
    }
}

fn main() {
    let mut s = Pisa { hole_size: 2 };
    println!("{}", s.hole_size);
    s.stretch();
    println!("{}", s.hole_size);
}
