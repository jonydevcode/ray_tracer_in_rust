use std::cmp::{max, min};

use crate::math_utils;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64, // 1 = point, 0 = vector
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    tuple: Tuple,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        math_utils::f64_equals(self.w, 1.0)
    }

    pub fn is_vector(&self) -> bool {
        math_utils::f64_equals(self.w, 0.0)
    }

    pub fn equals(&self, other: &Self) -> bool {
        math_utils::f64_equals(self.x, other.x)
            && math_utils::f64_equals(self.y, other.y)
            && math_utils::f64_equals(self.z, other.z)
            && math_utils::f64_equals(self.w, other.w)
    }

    pub fn add(&self, other: Self) -> Self {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn minus(&self, other: Self) -> Self {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    pub fn negate(&self) -> Self {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    pub fn multiply(&self, val: f64) -> Self {
        Tuple {
            x: self.x * val,
            y: self.y * val,
            z: self.z * val,
            w: self.w * val,
        }
    }

    pub fn divide(&self, val: f64) -> Self {
        Tuple {
            x: self.x / val,
            y: self.y / val,
            z: self.z / val,
            w: self.w / val,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self.divide(self.magnitude())
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Self) -> Self {
        Tuple::new_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn to_vec(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z, self.w]
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            tuple: Tuple::new(r, g, b, 0.0),
        }
    }

    pub fn r(&self) -> f64 {
        self.tuple.x
    }

    pub fn g(&self) -> f64 {
        self.tuple.y
    }

    pub fn b(&self) -> f64 {
        self.tuple.z
    }

    pub fn add(&self, other: Self) -> Self {
        Color {
            tuple: self.tuple.add(other.tuple),
        }
    }

    pub fn minus(&self, other: Self) -> Self {
        Color {
            tuple: self.tuple.minus(other.tuple),
        }
    }

    pub fn scale(&self, val: f64) -> Self {
        Color {
            tuple: self.tuple.multiply(val),
        }
    }

    pub fn multiply(&self, other: Self) -> Self {
        Color::new(
            self.r() * other.r(),
            self.g() * other.g(),
            self.b() * other.b(),
        )
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.tuple.equals(&other.tuple)
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn ppm_str(&self) -> (usize, usize, usize) {
        let r = min(max((self.r() * 255.0).round() as usize, 0), 255);
        let g = min(max((self.g() * 255.0).round() as usize, 0), 255);
        let b = min(max((self.b() * 255.0).round() as usize, 0), 255);
        (r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_point() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert!(t.is_point());
        assert!(!t.is_vector());
    }

    #[test]
    fn is_vector() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert!(!t.is_point());
        assert!(t.is_vector());
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let a2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };
        let result = Tuple {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0,
        };
        assert!(a1.add(a2).equals(&result));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::new_point(3.0, 2.0, 1.0);
        let p2 = Tuple::new_point(5.0, 6.0, 7.0);
        let result = Tuple::new_vector(-2.0, -4.0, -6.0);
        assert!(p1.minus(p2).equals(&result));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Tuple::new_point(3.0, 2.0, 1.0);
        let v = Tuple::new_vector(5.0, 6.0, 7.0);
        let result = Tuple::new_point(-2.0, -4.0, -6.0);
        assert!(p.minus(v).equals(&result));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let v2 = Tuple::new_vector(5.0, 6.0, 7.0);
        let result = Tuple::new_vector(-2.0, -4.0, -6.0);
        assert!(v1.minus(v2).equals(&result));
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = Tuple::new_vector(0.0, 0.0, 0.0);
        let v = Tuple::new_vector(1.0, -2.0, 3.0);
        let result = Tuple::new_vector(-1.0, 2.0, -3.0);
        assert!(zero.minus(v).equals(&result));
    }

    #[test]
    fn negating_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        assert!(a.negate().equals(&result));
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = Tuple::new(3.5, -7.0, 10.5, -14.0);
        assert!(a.multiply(3.5).equals(&result));
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert!(a.divide(2.0).equals(&result));
    }

    #[test]
    fn magnitude_tests() {
        let v = Tuple::new_vector(1.0, 0.0, 0.0);
        assert!(math_utils::f64_equals(v.magnitude(), 1.0));
        let v = Tuple::new_vector(0.0, 1.0, 0.0);
        assert!(math_utils::f64_equals(v.magnitude(), 1.0));
        let v = Tuple::new_vector(0.0, 0.0, 1.0);
        assert!(math_utils::f64_equals(v.magnitude(), 1.0));
        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        assert!(math_utils::f64_equals(v.magnitude(), (14.0 as f64).sqrt()));
        let v = Tuple::new_vector(-1.0, -2.0, -3.0);
        assert!(math_utils::f64_equals(v.magnitude(), (14.0 as f64).sqrt()));
    }

    #[test]
    fn normalization_tests() {
        let v = Tuple::new_vector(4.0, 0.0, 0.0);
        assert!(v.normalize().equals(&Tuple::new_vector(1.0, 0.0, 0.0)));
        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        assert!(v
            .normalize()
            .equals(&Tuple::new_vector(0.26726, 0.53452, 0.80178)));
        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        assert!(math_utils::f64_equals(v.normalize().magnitude(), 1.0));
    }

    #[test]
    fn dot_product_tests() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);
        assert!(math_utils::f64_equals(a.dot(b), 20.0));
    }

    #[test]
    fn cross_product_tests() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);
        assert!(a.cross(b).equals(&Tuple::new_vector(-1.0, 2.0, -1.0)));
        assert!(b.cross(a).equals(&Tuple::new_vector(1.0, -2.0, 1.0)));
    }

    #[test]
    fn color_operations() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!(c1.add(c2).equals(&Color::new(1.6, 0.7, 1.0)));

        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!(c1.minus(c2).equals(&Color::new(0.2, 0.5, 0.5)));

        let c = Color::new(0.2, 0.3, 0.4);
        assert!(c.scale(2.0).equals(&Color::new(0.4, 0.6, 0.8)));

        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert!(c1.multiply(c2).equals(&Color::new(0.9, 0.2, 0.04)));
    }

    // #[test]
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };
    //     assert!(larger.can_hold(&smaller));
    // }
}
