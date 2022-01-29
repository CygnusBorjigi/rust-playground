#[derive(Debug)]
struct Cube {
    weidth: u64,
    length: u64,
    height: u64
}

impl Cube {
    fn volume(&self) -> u64 {
        return self.weidth * self.length * self.height;
    }
}

fn create_cude(weidth: u64, length:u64, height: u64) -> Cube {
    Cube{
        weidth,
        length,
        height
    }
}

pub fn run_basic_struct () {
    let cube1 = create_cude(1, 2, 3);
    println!("{:#?}", cube1);
    println!("The volume is: {:?}", cube1.volume());
}

