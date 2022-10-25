use hilbert_curve_rust::CoordinateValue;
use hilbert_curve_rust::HilbertCurveAlgorithm;

#[test]
fn index_to_point_first_order_index_0() {
    let hilbert_curve = HilbertCurveAlgorithm::new(1);
    let point = hilbert_curve.index_to_point(0);
    assert_eq!(point.x, 0);
    assert_eq!(point.y, 0);
}
#[test]
fn index_to_point_first_order_index_1() {
    let hilbert_curve = HilbertCurveAlgorithm::new(1);
    let point = hilbert_curve.index_to_point(1);
    assert_eq!(point.x, 0);
    assert_eq!(point.y, 1);
}
#[test]
fn index_to_point_first_order_index_2() {
    let hilbert_curve = HilbertCurveAlgorithm::new(1);
    let point = hilbert_curve.index_to_point(2);
    assert_eq!(point.x, 1);
    assert_eq!(point.y, 1);
}
#[test]
fn index_to_point_first_order_index_3() {
    let hilbert_curve = HilbertCurveAlgorithm::new(1);
    let point = hilbert_curve.index_to_point(3);
    assert_eq!(point.x, 1);
    assert_eq!(point.y, 0);
}
#[test]
#[should_panic(
    expected = "The index is above the supported amount of space the current order support. Reduce the index or increase the order."
)]
fn index_to_point_second_order_index_4() {
    let hilbert_curve = HilbertCurveAlgorithm::new(1);
    hilbert_curve.index_to_point(4);
}
// -------------------------
#[test]
fn index_to_point_second_order_index_0() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    let point = hilbert_curve.index_to_point(0);
    assert_eq!(point.x, 0);
    assert_eq!(point.y, 0);
}
#[test]
fn index_to_point_second_order_index_1() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    let point = hilbert_curve.index_to_point(1);
    assert_eq!(point.x, 1);
    assert_eq!(point.y, 0);
}
#[test]
fn index_to_point_second_order_index_2() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    let point = hilbert_curve.index_to_point(2);
    assert_eq!(point.x, 1);
    assert_eq!(point.y, 1);
}
#[test]
fn index_to_point_second_order_index_7() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    let point = hilbert_curve.index_to_point(7);
    assert_eq!(point.x, 1);
    assert_eq!(point.y, 2);
}
#[test]
fn index_to_point_second_order_index_11() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    let point = hilbert_curve.index_to_point(11);
    assert_eq!(point.x, 3);
    assert_eq!(point.y, 2);
}
#[test]
fn index_to_point_second_order_index_15() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    let point = hilbert_curve.index_to_point(15);
    assert_eq!(point.x, 3);
    assert_eq!(point.y, 0);
}
#[test]
#[should_panic(
    expected = "The index is above the supported amount of space the current order support. Reduce the index or increase the order."
)]
fn index_to_point_second_order_index_16() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    hilbert_curve.index_to_point(16);
}

// -------------------------
#[test]
fn index_to_point_third_order_index_0() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    let point = hilbert_curve.index_to_point(0);
    assert_eq!(point.x, 0);
    assert_eq!(point.y, 0);
}
#[test]
fn index_to_point_third_order_index_1() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    let point = hilbert_curve.index_to_point(1);
    assert_eq!(point.x, 0);
    assert_eq!(point.y, 1);
}
#[test]
fn index_to_point_third_order_index_2() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    let point = hilbert_curve.index_to_point(2);
    assert_eq!(point.x, 1);
    assert_eq!(point.y, 1);
}
#[test]
fn index_to_point_third_order_index_7() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    let point = hilbert_curve.index_to_point(7);
    assert_eq!(point.x, 2);
    assert_eq!(point.y, 1);
}
#[test]
fn index_to_point_third_order_index_11() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    let point = hilbert_curve.index_to_point(11);
    assert_eq!(point.x, 2);
    assert_eq!(point.y, 3);
}
#[test]
fn index_to_point_third_order_index_15() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    let point = hilbert_curve.index_to_point(15);
    assert_eq!(point.x, 0);
    assert_eq!(point.y, 3);
}
#[test]
#[should_panic(
    expected = "The index is above the supported amount of space the current order support. Reduce the index or increase the order."
)]
fn index_to_point_thid_order_index_16() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    hilbert_curve.index_to_point(64);
}
// -------------------------
#[test]
fn point_to_index_first_order_point_0_0() {
    let hilbert_curve = HilbertCurveAlgorithm::new(1);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 0, y: 0 });
    assert_eq!(index, 0);
}
#[test]
fn point_to_index_first_order_point_0_1() {
    let hilbert_curve = HilbertCurveAlgorithm::new(1);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 0, y: 1 });
    assert_eq!(index, 1);
}
#[test]
fn point_to_index_first_order_point_1_1() {
    let hilbert_curve = HilbertCurveAlgorithm::new(1);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 1, y: 1 });
    assert_eq!(index, 2);
}
#[test]
fn point_to_index_first_order_point_1_0() {
    let hilbert_curve = HilbertCurveAlgorithm::new(1);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 1, y: 0 });
    assert_eq!(index, 3);
}
#[test]
#[should_panic(expected = "The point must be in range with the order")]
fn point_to_index_first_order_point_2_2() {
    let hilbert_curve = HilbertCurveAlgorithm::new(1);
    hilbert_curve.point_to_index(CoordinateValue { x: 2, y: 2 });
}
// -------------------------
#[test]
fn point_to_index_second_order_point_0_0() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 0, y: 0 });
    assert_eq!(index, 0);
}
#[test]
fn point_to_index_second_order_point_1_0() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 1, y: 0 });
    assert_eq!(index, 1);
}
#[test]
fn point_to_index_second_order_point_1_1() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 1, y: 1 });
    assert_eq!(index, 2);
}
#[test]
fn point_to_index_second_order_point_1_2() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 1, y: 2 });
    assert_eq!(index, 7);
}
#[test]
fn point_to_index_second_order_point_3_2() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 3, y: 2 });
    assert_eq!(index, 11);
}
#[test]
fn point_to_index_second_order_point_3_0() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 3, y: 0 });
    assert_eq!(index, 15);
}
#[test]
#[should_panic(expected = "The point must be in range with the order")]
fn point_to_index_second_order_point_4_4() {
    let hilbert_curve = HilbertCurveAlgorithm::new(2);
    hilbert_curve.point_to_index(CoordinateValue { x: 4, y: 4 });
}
// -------------------------
#[test]
fn point_to_index_third_order_point_0_0() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 0, y: 0 });
    assert_eq!(index, 0);
}
#[test]
fn point_to_index_third_order_point_1_0() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 0, y: 1 });
    assert_eq!(index, 1);
}
#[test]
fn point_to_index_third_order_point_1_1() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 1, y: 1 });
    assert_eq!(index, 2);
}
#[test]
fn point_to_index_third_order_point_1_2() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 2, y: 1 });
    assert_eq!(index, 7);
}
#[test]
fn point_to_index_third_order_point_3_2() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 2, y: 3 });
    assert_eq!(index, 11);
}
#[test]
fn point_to_index_third_order_point_3_0() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    let index = hilbert_curve.point_to_index(CoordinateValue { x: 0, y: 3 });
    assert_eq!(index, 15);
}
#[test]
#[should_panic(expected = "The point must be in range with the order")]
fn point_to_index_third_order_point_5_5() {
    let hilbert_curve = HilbertCurveAlgorithm::new(3);
    hilbert_curve.point_to_index(CoordinateValue { x: 8, y: 8 });
}
// -------------------------