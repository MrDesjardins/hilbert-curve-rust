use crate::coordinate_value::CoordinateValue;
use std::mem;

pub struct HilbertCurveAlgorithm {
    order: u16,
}

impl HilbertCurveAlgorithm {
    pub fn new(order: u16) -> Self {
        Self { order }
    }

    pub fn index_to_point(&self, index: u32) -> CoordinateValue {
        let number_row = u32::pow(2, self.order.into());
        let maximum_data_size = u32::pow(number_row, 2);
        if index >= maximum_data_size {
            panic!("The index is above the supported amount of space the current order support. Reduce the index or increase the order.");
        }

        let mut point = CoordinateValue { x: 0, y: 0 };
        let mut rx: u32;
        let mut ry: u32;
        let mut order_index: u32 = 1;
        let mut quadrant: u32 = index;

        while order_index < number_row {
            rx = self.get_rx(quadrant);
            ry = self.get_ry(quadrant, rx);
            HilbertCurveAlgorithm::rotate_point(&mut point, rx, ry, order_index); // Rotate depending on rx and ry value
            HilbertCurveAlgorithm::move_point(&mut point, rx, ry, order_index);
            quadrant = quadrant / 4; // 4 point per quadrant, hence we jump by 4
            order_index = order_index * 2; // Each order double the size of element per row (and column)
        }
        return point;
    }

    pub fn point_to_index(&self, point: CoordinateValue) -> u32 {
        let number_of_row = u32::pow(2, self.order.into());
        if point.x >= number_of_row || point.y >= number_of_row {
            panic!("The point must be in range with the order");
        }
        let mut rx: u32 = 0;
        let mut ry: u32 = 0;
        let mut index: u32 = 0;

        let mut row_index = number_of_row / 2;
        let mut new_point = point.clone(); // Ensure we are not mutating the original
        while row_index > 0 {
            HilbertCurveAlgorithm::update_rx_from_point(&mut rx, new_point, row_index);
            HilbertCurveAlgorithm::update_ry_from_point(&mut ry, new_point, row_index);
            index += HilbertCurveAlgorithm::get_new_index_from_rows(row_index, rx, ry);
            HilbertCurveAlgorithm::rotate_point(&mut new_point, rx, ry, number_of_row);
            row_index /= 2;
        }

        return index;
    }

    fn get_rx(&self, quadrant: u32) -> u32 {
        return 1 & (quadrant / 2);
    }
    fn get_ry(&self, quadrant: u32, rx: u32) -> u32 {
        let asd: u32 = quadrant ^ rx;
        let and_op: u32 = 1 & asd;
        return and_op;
    }
    fn rotate_point(mut point: &mut CoordinateValue, rx: u32, ry: u32, number_columns: u32) -> () {
        if ry == 0 {
            if rx == 1 {
                point.x = number_columns as u32 - 1 - point.x as u32;
                point.y = number_columns as u32 - 1 - point.y as u32;
            }
            mem::swap(&mut point.x, &mut point.y);
        }
    }
    fn move_point(mut point: &mut CoordinateValue, rx: u32, ry: u32, order_index: u32) -> () {
        point.x = point.x + order_index * rx;
        point.y = point.y + order_index * ry;
    }

    fn update_rx_from_point(rx: &mut u32, point: CoordinateValue, order_index: u32) -> () {
        *rx = HilbertCurveAlgorithm::update_point_value_from_number(point.x, order_index);
    }

    fn update_ry_from_point(ry: &mut u32, point: CoordinateValue, order_index: u32) -> () {
        *ry = HilbertCurveAlgorithm::update_point_value_from_number(point.y, order_index);
    }
    fn update_point_value_from_number(number_n: u32, order_index: u32) -> u32 {
        let and_result = number_n & order_index; // 0, 1, 2
        return if and_result > 0 { 1 } else { 0 };
    }
    fn get_new_index_from_rows(rows_index: u32, rx: u32, ry: u32) -> u32 {
        return rows_index * rows_index * ((3 * rx) ^ ry);
    }
    pub fn offset_point(&self, point: CoordinateValue, projection_width: u32) -> CoordinateValue {
        let number_of_row: u32 = u32::pow(2, self.order as u32);
        let len = projection_width / number_of_row;
        return CoordinateValue {
            x: point.x * len + len / 2,
            y: point.y * len + len / 2,
        };
    }
    pub fn deoffset_point(&self, point: CoordinateValue, projection_width: u32) -> CoordinateValue {
        let number_of_row: u32 = u32::pow(2, self.order as u32);
        let len = projection_width / number_of_row;
        return CoordinateValue {
            x: point.x / len,
            y: point.y / len,
        };
    }
}

#[cfg(test)]
mod test_get_rx {
    use super::*;

    #[test]
    fn internal_get_rx() {
        let hilbert_curve = HilbertCurveAlgorithm::new(0);
        assert_eq!(hilbert_curve.get_rx(0), 0);
        assert_eq!(hilbert_curve.get_rx(1), 0);
        assert_eq!(hilbert_curve.get_rx(2), 1);
        assert_eq!(hilbert_curve.get_rx(3), 1);
        assert_eq!(hilbert_curve.get_rx(4), 0);
        assert_eq!(hilbert_curve.get_rx(5), 0);
        assert_eq!(hilbert_curve.get_rx(6), 1);
    }
}

#[cfg(test)]
mod test_get_ry {
    use super::*;

    #[test]
    fn internal_get_ry_with_rx_0() {
        let hilbert_curve = HilbertCurveAlgorithm::new(0);
        assert_eq!(hilbert_curve.get_ry(0, 0), 0);
        assert_eq!(hilbert_curve.get_ry(1, 0), 1);
        assert_eq!(hilbert_curve.get_ry(2, 0), 0);
        assert_eq!(hilbert_curve.get_ry(3, 0), 1);
        assert_eq!(hilbert_curve.get_ry(4, 0), 0);
        assert_eq!(hilbert_curve.get_ry(5, 0), 1);
        assert_eq!(hilbert_curve.get_ry(6, 0), 0);
    }

    #[test]
    fn internal_get_ry_with_rx_1() {
        let hilbert_curve = HilbertCurveAlgorithm::new(0);
        assert_eq!(hilbert_curve.get_ry(0, 1), 1);
        assert_eq!(hilbert_curve.get_ry(1, 1), 0);
        assert_eq!(hilbert_curve.get_ry(2, 1), 1);
        assert_eq!(hilbert_curve.get_ry(3, 1), 0);
        assert_eq!(hilbert_curve.get_ry(4, 1), 1);
        assert_eq!(hilbert_curve.get_ry(5, 1), 0);
        assert_eq!(hilbert_curve.get_ry(6, 1), 1);
    }
}

#[cfg(test)]
mod test_move_point {
    use super::*;

    #[test]
    fn internal_move_point_order_1_rx_0_no_move() {
        let order = 1;
        let rx = 0;
        let ry = 0;
        let mut point = CoordinateValue { x: 123, y: 456 };
        HilbertCurveAlgorithm::move_point(&mut point, rx, ry, order);
        assert_eq!(point.x, 123);
        assert_eq!(point.y, 456);
    }

    #[test]
    fn internal_move_point_order_1_rx_1_move() {
        let order = 1;
        let rx = 1;
        let ry = 0;
        let mut point = CoordinateValue { x: 123, y: 456 };
        HilbertCurveAlgorithm::move_point(&mut point, rx, ry, order);
        assert_eq!(point.x, 124);
        assert_eq!(point.y, 456);
    }

    #[test]
    fn internal_move_point_order_1_ry_0_no_move() {
        let order = 1;
        let rx = 0;
        let ry = 0;
        let mut point = CoordinateValue { x: 123, y: 456 };
        HilbertCurveAlgorithm::move_point(&mut point, rx, ry, order);
        assert_eq!(point.x, 123);
        assert_eq!(point.y, 456);
    }

    #[test]
    fn internal_move_point_order_1_ry_1_move() {
        let order = 1;
        let rx = 0;
        let ry = 1;
        let mut point = CoordinateValue { x: 123, y: 456 };
        HilbertCurveAlgorithm::move_point(&mut point, rx, ry, order);
        assert_eq!(point.x, 123);
        assert_eq!(point.y, 457);
    }
    #[test]
    fn internal_move_point_order_2_rx_0_no_move() {
        let order = 2;
        let rx = 0;
        let ry = 0;
        let mut point = CoordinateValue { x: 123, y: 456 };
        HilbertCurveAlgorithm::move_point(&mut point, rx, ry, order);
        assert_eq!(point.x, 123);
        assert_eq!(point.y, 456);
    }

    #[test]
    fn internal_move_point_order_2_rx_1_move() {
        let order = 2;
        let rx = 1;
        let ry = 0;
        let mut point = CoordinateValue { x: 123, y: 456 };
        HilbertCurveAlgorithm::move_point(&mut point, rx, ry, order);
        assert_eq!(point.x, 125);
        assert_eq!(point.y, 456);
    }

    #[test]
    fn internal_move_point_order_2_ry_0_no_move() {
        let order = 2;
        let rx = 0;
        let ry = 0;
        let mut point = CoordinateValue { x: 123, y: 456 };
        HilbertCurveAlgorithm::move_point(&mut point, rx, ry, order);
        assert_eq!(point.x, 123);
        assert_eq!(point.y, 456);
    }

    #[test]
    fn internal_move_point_order_2_ry_1_move() {
        let order = 2;
        let rx = 0;
        let ry = 1;
        let mut point = CoordinateValue { x: 123, y: 456 };
        HilbertCurveAlgorithm::move_point(&mut point, rx, ry, order);
        assert_eq!(point.x, 123);
        assert_eq!(point.y, 458);
    }
}

#[cfg(test)]
mod test_get_point_value_from_number {
    use super::*;
    #[test]
    fn internal_get_point_value_from_number_order1() {
        assert_eq!(
            0,
            HilbertCurveAlgorithm::update_point_value_from_number(0, 1)
        );
        assert_eq!(
            1,
            HilbertCurveAlgorithm::update_point_value_from_number(1, 1)
        );
        assert_eq!(
            0,
            HilbertCurveAlgorithm::update_point_value_from_number(2, 1)
        );
        assert_eq!(
            1,
            HilbertCurveAlgorithm::update_point_value_from_number(3, 1)
        );
    }
    #[test]
    fn internal_get_point_value_from_number_order2() {
        assert_eq!(
            0,
            HilbertCurveAlgorithm::update_point_value_from_number(0, 2)
        );
        assert_eq!(
            0,
            HilbertCurveAlgorithm::update_point_value_from_number(1, 2)
        );
        assert_eq!(
            1,
            HilbertCurveAlgorithm::update_point_value_from_number(2, 2)
        );
        assert_eq!(
            1,
            HilbertCurveAlgorithm::update_point_value_from_number(3, 2)
        );
        assert_eq!(
            0,
            HilbertCurveAlgorithm::update_point_value_from_number(4, 2)
        );
        assert_eq!(
            0,
            HilbertCurveAlgorithm::update_point_value_from_number(5, 2)
        );
        assert_eq!(
            1,
            HilbertCurveAlgorithm::update_point_value_from_number(6, 2)
        );
        assert_eq!(
            1,
            HilbertCurveAlgorithm::update_point_value_from_number(7, 2)
        );
    }
    #[test]
    fn internal_get_point_value_from_number_order4() {
        assert_eq!(
            0,
            HilbertCurveAlgorithm::update_point_value_from_number(0, 4)
        );
        assert_eq!(
            0,
            HilbertCurveAlgorithm::update_point_value_from_number(1, 4)
        );
        assert_eq!(
            0,
            HilbertCurveAlgorithm::update_point_value_from_number(2, 4)
        );
        assert_eq!(
            0,
            HilbertCurveAlgorithm::update_point_value_from_number(3, 4)
        );
        assert_eq!(
            1,
            HilbertCurveAlgorithm::update_point_value_from_number(4, 4)
        );
        assert_eq!(
            1,
            HilbertCurveAlgorithm::update_point_value_from_number(5, 4)
        );
        assert_eq!(
            1,
            HilbertCurveAlgorithm::update_point_value_from_number(6, 4)
        );
        assert_eq!(
            1,
            HilbertCurveAlgorithm::update_point_value_from_number(7, 4)
        );
    }
}

#[cfg(test)]
mod test_rotate_point {
    use super::*;
    #[test]
    fn internal_rotate_point_0_0_col1_x_0_y_0() {
        let mut coordinate = CoordinateValue { x: 0, y: 0 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 0, 0, 1);
        assert_eq!(0, coordinate.x, "X value is wrong");
        assert_eq!(0, coordinate.y, "Y value is wrong");
    }
    #[test]
    fn internal_rotate_point_0_0_col1_x_0_y_1() {
        let mut coordinate = CoordinateValue { x: 0, y: 0 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 0, 1, 1);
        assert_eq!(0, coordinate.x, "X value is wrong");
        assert_eq!(0, coordinate.y, "Y value is wrong");
    }
    #[test]
    fn internal_rotate_point_0_0_col1_x_1_y_0() {
        let mut coordinate = CoordinateValue { x: 0, y: 0 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 1, 0, 1);
        assert_eq!(0, coordinate.x, "X value is wrong");
        assert_eq!(0, coordinate.y, "Y value is wrong");
    }
    #[test]
    fn internal_rotate_point_1_1_col1_x_1_y_0() {
        let mut coordinate = CoordinateValue { x: 1, y: 1 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 1, 0, 2);
        assert_eq!(0, coordinate.x, "X value is wrong");
        assert_eq!(0, coordinate.y, "Y value is wrong");
    }
    #[test]
    fn internal_rotate_point_0_0_col1_x_1_y_1() {
        let mut coordinate = CoordinateValue { x: 0, y: 0 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 1, 1, 1);
        assert_eq!(0, coordinate.x, "X value is wrong");
        assert_eq!(0, coordinate.y, "Y value is wrong");
    }
    #[test]
    fn internal_rotate_point_1_1_col1_x_1_y_1() {
        let mut coordinate = CoordinateValue { x: 1, y: 1 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 1, 1, 2);
        assert_eq!(1, coordinate.x, "X value is wrong");
        assert_eq!(1, coordinate.y, "Y value is wrong");
    }
    #[test]
    fn internal_rotate_point_1_2_col1_x_0_y_1() {
        let mut coordinate = CoordinateValue { x: 1, y: 2 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 0, 1, 2);
        assert_eq!(1, coordinate.x, "X value is wrong");
        assert_eq!(2, coordinate.y, "Y value is wrong");
    }
    #[test]
    fn internal_rotate_point_1_2_col1_x_0_y_0() {
        let mut coordinate = CoordinateValue { x: 1, y: 2 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 0, 0, 2);
        assert_eq!(2, coordinate.x, "X value is wrong");
        assert_eq!(1, coordinate.y, "Y value is wrong");
    }

    #[test]
    fn internal_rotate_point_numbercolumn_8_point_0_0_x_0_y_0() {
        let mut coordinate = CoordinateValue { x: 0, y: 0 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 0, 0, 8);
        assert_eq!(0, coordinate.x, "X value is wrong");
        assert_eq!(0, coordinate.y, "Y value is wrong");
    }
    #[test]
    fn internal_rotate_point_numbercolumn_8_point_0_0_x_0_y_1() {
        let mut coordinate = CoordinateValue { x: 0, y: 0 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 0, 1, 8);
        assert_eq!(0, coordinate.x, "X value is wrong");
        assert_eq!(0, coordinate.y, "Y value is wrong");
    }
    #[test]
    fn internal_rotate_point_numbercolumn_8_point_0_0_x_1_y_0() {
        let mut coordinate = CoordinateValue { x: 0, y: 0 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 1, 0, 8);
        assert_eq!(7, coordinate.x, "X value is wrong");
        assert_eq!(7, coordinate.y, "Y value is wrong");
    }
    #[test]
    fn internal_rotate_point_numbercolumn_8_point_1_1_x_1_y_0() {
        let mut coordinate = CoordinateValue { x: 1, y: 1 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 1, 0, 8);
        assert_eq!(6, coordinate.x, "X value is wrong");
        assert_eq!(6, coordinate.y, "Y value is wrong");
    }
    #[test]
    fn internal_rotate_point_numbercolumn_8_point_0_0_x_1_y_1() {
        let mut coordinate = CoordinateValue { x: 0, y: 0 };
        HilbertCurveAlgorithm::rotate_point(&mut coordinate, 1, 1, 8);
        assert_eq!(0, coordinate.x, "X value is wrong");
        assert_eq!(0, coordinate.y, "Y value is wrong");
    }
}

#[cfg(test)]
mod test_get_new_index_from_rows {
    use super::*;
    #[test]
    fn get_new_index_from_rows_index_1_x_0_y_0() {
        let result = HilbertCurveAlgorithm::get_new_index_from_rows(1, 0, 0);
        assert_eq!(0, result);
    }
    #[test]
    fn get_new_index_from_rows_index_1_x_0_y_1() {
        let result = HilbertCurveAlgorithm::get_new_index_from_rows(1, 0, 1);
        assert_eq!(1, result);
    }
    #[test]
    fn get_new_index_from_rows_index_1_x_1_y_0() {
        let result = HilbertCurveAlgorithm::get_new_index_from_rows(1, 1, 0);
        assert_eq!(3, result);
    }
    #[test]
    fn get_new_index_from_rows_index_1_x_1_y_1() {
        let result = HilbertCurveAlgorithm::get_new_index_from_rows(1, 1, 1);
        assert_eq!(2, result);
    }
    #[test]
    fn get_new_index_from_rows_index_2_x_0_y_0() {
        let result = HilbertCurveAlgorithm::get_new_index_from_rows(2, 0, 0);
        assert_eq!(0, result);
    }
    #[test]
    fn get_new_index_from_rows_index_2_x_0_y_1() {
        let result = HilbertCurveAlgorithm::get_new_index_from_rows(2, 0, 1);
        assert_eq!(4, result);
    }
    #[test]
    fn get_new_index_from_rows_index_2_x_1_y_0() {
        let result = HilbertCurveAlgorithm::get_new_index_from_rows(2, 1, 0);
        assert_eq!(12, result);
    }
    #[test]
    fn get_new_index_from_rows_index_2_x_1_y_1() {
        let result = HilbertCurveAlgorithm::get_new_index_from_rows(2, 1, 1);
        assert_eq!(8, result);
    }
}

#[cfg(test)]
mod test_offset_point {
    use super::*;

    #[test]
    fn test_offset_point_positive() {
        let hilbert_curve = HilbertCurveAlgorithm::new(3);
        let result = hilbert_curve.offset_point(CoordinateValue { x: 0, y: 3 }, 128);
        assert_eq!(8, result.x, "X value is wrong");
        assert_eq!(56, result.y, "Y value is wrong");
    }
}

#[cfg(test)]
mod test_deoffset_point {
    use super::*;

    #[test]
    fn test_offset_point_positive() {
        let hilbert_curve = HilbertCurveAlgorithm::new(3);
        let result = hilbert_curve.deoffset_point(CoordinateValue { x: 8, y: 56 }, 128);
        assert_eq!(0, result.x, "X value is wrong");
        assert_eq!(3, result.y, "Y value is wrong");
    }
}
