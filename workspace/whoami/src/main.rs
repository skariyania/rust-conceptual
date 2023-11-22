use whoami::WhoAmI;
use whoami_derive::WhoAmI;

#[derive(WhoAmI)]
struct Pancakes;

fn main() {
    Pancakes::whoami();
}
