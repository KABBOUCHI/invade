use invade::Invade;

#[derive(Debug, Invade)]
struct Counter {
    count: u32,
}

impl Counter {
    pub fn inc(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut counter = Counter { count: 69 };

    println!("count: {:?}", counter.invade().get::<u32>("count"));
    println!("count: {:?}", counter.invade_get::<u32>("count"));

    counter.invade().set("count", 42_u32);
    counter.invade_set("count", 42_u32);

    println!("count: {:?}", counter.invade_get::<u32>("count"));

    counter.invade_call("inc");

    println!("count: {}", counter.count);
}
