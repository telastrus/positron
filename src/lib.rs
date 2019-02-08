extern crate rppal;

use rppal::i2c::I2c;

mod fieldmap;
pub use fieldmap::Fieldmap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


struct Positron {
    lidar: I2c,
    map: Fieldmap
}

impl Positron {

}