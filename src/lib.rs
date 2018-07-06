#[cfg(test)] extern crate utilities;

#[derive(Clone)]
pub struct Sphere {
    pub values: i32,
}

impl Sphere {
    pub fn new(v: i32) -> Sphere {
        let values = v;
        Sphere {values}
    }
}

pub fn compute_sphere(sphere: Sphere) -> i32 {
    sphere.values
}

#[cfg(test)]
mod tests {
    use super::*;

    use utilities::get_default_sphere;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_compute_sphere() {
        let res = 10;
        assert_eq!(compute_sphere(get_default_sphere()), res);
    }
}
