use polymorphism::{road_trip, Sedan, SUV};

trait LandCapable {
    fn drive(&self);
}

fn main() {
    let suv = SUV;
    let sedan = Sedan;
    road_trip(&suv);
    road_trip(&sedan);
}
