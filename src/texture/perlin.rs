use rand::{seq::SliceRandom, Rng};

use crate::{Point3, Vec3};

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

fn corner_iterator() -> impl Iterator<Item = (usize, usize, usize)> {
    (0..2 as usize).into_iter().flat_map(move |i| {
        (0..2 as usize)
            .into_iter()
            .flat_map(move |j| (0..2 as usize).into_iter().map(move |k| (i, j, k)))
    })
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

    const MAX_INDEX: usize = Perlin::POINT_COUNT - 1;
    fn trilinear_interpolation(&self, point: &Point3, intermediate: Vec3<f64>) -> f64 {
        let floor = point.apply(|x| x.floor() as isize as usize);
        let mut result: f64 = 0.0;
        let interp = |t: f64, x: f64| t * x + (1.0 - t) * (1.0 - x);

        for (i, j, k) in corner_iterator() {
            let index = self.perm_x[(floor.x() + i) & Self::MAX_INDEX]
                ^ self.perm_y[(floor.y() + j) & Self::MAX_INDEX]
                ^ self.perm_z[(floor.z() + k) & Self::MAX_INDEX];
            let corner = self.randoms[index];
            result += interp(intermediate.x(), i as f64)
                * interp(intermediate.y(), j as f64)
                * interp(intermediate.z(), k as f64)
                * corner;
        }

        result
    }

    pub fn noise(&self, point: &Point3) -> f64 {
        assert!(Self::POINT_COUNT.is_power_of_two());

        let intermediate = point.apply(|x| x - x.floor());
        self.trilinear_interpolation(point, intermediate)
    }
}
