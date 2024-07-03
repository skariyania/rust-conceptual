pub trait LandCapable {
    fn drive(&self);
}

pub struct Sedan;
impl LandCapable for Sedan {
    fn drive(&self) {
        println!("I am driving Sedan")
    }
}

pub struct SUV;
impl LandCapable for SUV {
    fn drive(&self) {
        println!("I am driving SUV")
    }
}

// this dyn refers to dynamic dispatch this is in contrast to static dispatch
// this is referred as fat pointers (one points to data of struct
//    and another points to v-table of struct)
// downsize of this approach is it occurs a runtime penalty
// to implement static dispatch we should be using below syntax instead
// fn road_trip(vehicle: &impl LandCapable) {
pub fn road_trip(vehicle: &dyn LandCapable) {
    vehicle.drive();
}
