# invade

## Setup

```bash
cargo add invade
```

## Usage

```rust
use invade::Invade;

#[derive(Invade)]
struct Counter {
    count: u32,
}

fn main() {
    let mut counter = Counter { count: 0 };
  
    println!("count: {:?}", counter.invade_get::<u32>("count"));

    counter.invade_set("count", 1_u32);

    assert_eq!(counter.count, 1);
}
```