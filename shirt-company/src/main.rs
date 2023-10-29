use shirt_company::{self, Inventory, ShirtColor};
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let sahil_pref = Some(ShirtColor::Red);

    let giveaway_sahil = store.giveaway(sahil_pref);
    println!(
        "The user sahil with preference {:?} gets {:?}",
        sahil_pref, giveaway_sahil
    );

    let anonymous_user_preference = None;
    let giveaway_anonymous = store.giveaway(anonymous_user_preference);
    println!(
        "The user with preference {:?} gets {:?}",
        anonymous_user_preference, giveaway_anonymous
    );
}
