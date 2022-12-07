use rand::{seq::SliceRandom, Rng};

use crate::Point3;

/// Perlin noise
///
/// A key part of Perlin noise is that it is repeatable: it takes a 3D point as input
/// and always returns the same randomish number. Nearby points return similar numbers.
/// Another important part of Perlin noise is that it be simple and fast, so it’s
/// usually done as a hack. I’ll build that hack up incrementally based on Andrew
/// Kensler’s description.
#[derive(Debug, Clone)]
pub struct Perlin {
    /// Permutation table
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
    /// Random numbers
    randoms: Vec<f64>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let range = 0..Self::POINT_COUNT;
        let randoms = range.clone().map(|_| rng.gen::<f64>()).collect();

        let mut perm = || {
            let mut vec = range.clone().collect::<Vec<_>>();
            vec.shuffle(&mut rng);
            vec
        };

        Self {
            perm_x: perm(),
            perm_y: perm(),
            perm_z: perm(),
            randoms,
        }
    }

    pub fn noise(&self, point: &Point3) -> f64 {
        const MAX_INDEX: usize = Perlin::POINT_COUNT - 1;
        assert!(Self::POINT_COUNT.is_power_of_two());

        // first convert to isize then usize to avoid overflow,
        // which cast negative f64 -1.0 to usize 0
        let to_index = |x: f64| ((x * 4.0) as isize as usize) & MAX_INDEX;

        let x_index = to_index(point.x());
        let y_index = to_index(point.y());
        let z_index = to_index(point.z());
        let random_index = self.perm_x[x_index] ^ self.perm_y[y_index] ^ self.perm_z[z_index];

        self.randoms[random_index]
    }
}
