use invade::invade;

#[derive(Debug)]
#[invade]
struct Counter {
    count: u32,
    title: String,
}

#[invade]
impl Counter {
    pub fn inc(&mut self) {
        self.count += 1;
    }

    pub fn dec(&mut self) {
        self.count -= 1;
    }
}

fn main() {
    let mut counter = Counter {
        count: 69,
        title: "Counter".to_string(),
    };

    println!("count: {:?}", counter.invade().get::<u32>("count"));
    println!("count: {:?}", counter.invade_get::<u32>("count"));

    counter.invade().set("count", 42_u32);
    counter.invade_set("count", 42_u32);

    println!("count: {:?}", counter.invade_get::<u32>("count"));

    counter.invade_call("inc", vec![]);
    counter.invade_call("inc", vec![]);
    counter.invade_call("dec", vec![]);
    counter.invade_call("inc", vec![]);

    println!("count: {}", counter.count);

    println!("title: {:?}", counter.invade_get::<String>("title"));

    // println!("methods: {:?}", counter.invaded_methods());
}
