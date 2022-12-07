use rand::seq::SliceRandom;

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
    /// Random vectors
    random_vectors: Vec<Vec3<f64>>,
}

fn corner_iterator() -> impl Iterator<Item = (usize, usize, usize)> {
    (0..2_usize).into_iter().flat_map(move |i| {
        (0..2_usize)
            .into_iter()
            .flat_map(move |j| (0..2_usize).into_iter().map(move |k| (i, j, k)))
    })
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let range = 0..Self::POINT_COUNT;
        let random_vectors = range
            .clone()
            .map(|_| Vec3::random(-1.0..1.0).normalized())
            .collect();

        let mut perm = || {
            let mut vec = range.clone().collect::<Vec<_>>();
            vec.shuffle(&mut rng);
            vec
        };

        Self {
            perm_x: perm(),
            perm_y: perm(),
            perm_z: perm(),
            random_vectors,
        }
    }

    const MAX_INDEX: usize = Perlin::POINT_COUNT - 1;
    fn perlin_interpolation(&self, point: &Point3, intermediate: Vec3<f64>) -> f64 {
        // Hermite cubic
        let smoothed = intermediate.apply(|x| x * x * (3.0 - 2.0 * x));

        let floor = point.apply(|x| x.floor() as isize as usize);
        let interp = |t: f64, x: f64| t * x + (1.0 - t) * (1.0 - x);

        let mut result: f64 = 0.0;

        for (i, j, k) in corner_iterator() {
            let index = self.perm_x[(floor.x() + i) & Self::MAX_INDEX]
                ^ self.perm_y[(floor.y() + j) & Self::MAX_INDEX]
                ^ self.perm_z[(floor.z() + k) & Self::MAX_INDEX];
            let corner = self.random_vectors[index];
            let weight = intermediate - Vec3::new(i as f64, j as f64, k as f64);

            result += interp(smoothed.x(), i as f64)
                * interp(smoothed.y(), j as f64)
                * interp(smoothed.z(), k as f64)
                * corner.dot(weight);
        }

        result
    }

    /// Get the noise value at a point
    pub fn noise(&self, point: &Point3) -> f64 {
        assert!(Self::POINT_COUNT.is_power_of_two());

        let intermediate = point.apply(|x| x - x.floor());
        self.perlin_interpolation(point, intermediate)
    }

    /// Get the turbulence value at a point, which a composite noise that has multiple
    /// summed frenquencies
    pub fn turbulence(&self, point: &Point3, depth: usize) -> f64 {
        let result = (0..depth)
            .fold((0.0, *point, 1.0), |(result, point, weight), _| {
                (
                    result + weight * self.noise(&point),
                    point * 2.0,
                    weight * 0.5,
                )
            })
            .0;

        result.abs()
    }
}
