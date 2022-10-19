use crate::coordinate_value::CoordinateValue;

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
            point = self.rotate_point(point.clone(), rx, ry, order_index); // Rotate depending on rx and ry value
            point = self.move_point(point.clone(), rx, ry, order_index);
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
        let mut rx: u32;
        let mut ry: u32;
        let mut index: u32 = 0;

        let mut row_index = number_of_row / 2;
        let mut new_point = point.clone();
        while row_index > 0 {
            rx = self.get_rx_from_point(new_point, row_index);
            ry = self.get_ry_from_point(new_point, row_index);
            index += self.get_new_index_from_rows(row_index, rx, ry);
            new_point = self.rotate_point(new_point, rx, ry, number_of_row);
            row_index = ((row_index as f32) / 2.0).floor() as u32;
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
    fn rotate_point(
        &self,
        mut point: CoordinateValue,
        rx: u32,
        ry: u32,
        number_columns: u32,
    ) -> CoordinateValue {
        if ry == 0 {
            if rx == 1 {
                point.x = number_columns - 1 - point.x;
                point.y = number_columns - 1 - point.y;
            }
            let tmp = point.x;
            point.x = point.y;
            point.y = tmp;
        }
        return point;
    }
    fn move_point(
        &self,
        mut point: CoordinateValue,
        rx: u32,
        ry: u32,
        order_index: u32,
    ) -> CoordinateValue {
        point.x += order_index * rx; // Move the x point from "a quadrant" size OR not (this is 0 or 1 multiplication)
        point.y += order_index * ry; // Move the x point from "a quadrant" size OR not (this is 0 or 1 multiplication)
        return point;
    }

    fn get_rx_from_point(&self, point: CoordinateValue, order_index: u32) -> u32 {
        return self.get_point_value_from_number(point.x, order_index);
    }

    fn get_ry_from_point(&self, point: CoordinateValue, order_index: u32) -> u32 {
        return self.get_point_value_from_number(point.y, order_index);
    }
    fn get_point_value_from_number(&self, number_n: u32, order_index: u32) -> u32 {
        let and_result = number_n & order_index; // 0, 1, 2
        return if and_result > 0 { 1 } else { 0 };
    }
    fn get_new_index_from_rows(&self, rows_index: u32, rx: u32, ry: u32) -> u32 {
        return rows_index * rows_index * ((3 * rx) ^ ry);
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
