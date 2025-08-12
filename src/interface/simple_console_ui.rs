use crate::domain::ViewUpdates;

pub struct SimpleConsoleUi {

}

impl ViewUpdates for SimpleConsoleUi {
    fn display_devices(&self, devices: &[crate::domain::Device]) {
        println!("{:?}", devices);
    }
}
