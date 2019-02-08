use std::vec::Vec;

struct Chunk {
    size: i16,
    l
}



pub struct Fieldmap {
    map: Vec<(f32, f32)>
}

impl Fieldmap {
    fn new() -> Fieldmap {
        Fieldmap { map: Vec::new() }
    }

    fn plot(&mut self, x: f32, y: f32) {
        self.map.push((x, y));
    }

    fn plot_angle(&mut self, r: f32, theta: f32) {

    }
}