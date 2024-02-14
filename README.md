# invade

## Setup

```bash
cargo add invade
```

## Usage

```rust
use invade::invade;

#[invade]
struct Counter {
    count: u32,
}

#[invade]
impl Counter {
    fn inc(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
  
    println!("count: {:?}", counter.invade_get::<u32>("count"));

    counter.invade_set("count", 1_u32);

    assert_eq!(counter.count, 1);

    counter.invade_call("inc", vec![]);

    assert_eq!(counter.count, 2);
}
```