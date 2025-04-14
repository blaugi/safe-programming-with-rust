pub struct Vector {
    pub components: Vec<f64>,
}

impl Vector {
    pub fn dimension(v: &Vector) -> usize {
        v.components.len()
    }

    pub fn magnitude(v: &Vector) -> f64 {
        let mut mag = 0.0;
        for c in &v.components {
            mag += c.powf(2.0);
        }
        mag.sqrt()
    }
}
