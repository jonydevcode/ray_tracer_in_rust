use float_cmp::{ApproxEq, F64Margin};

pub const EPSILON: f64 = 0.00001;

pub const FLOAT_CMP_MARGIN: F64Margin = F64Margin {
    epsilon: EPSILON,
    ulps: 16,
};

pub fn f64_equals(x: f64, y: f64) -> bool {
    x.approx_eq(y, FLOAT_CMP_MARGIN)
}
