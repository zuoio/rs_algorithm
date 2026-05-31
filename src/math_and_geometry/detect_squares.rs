use std::collections::HashMap;

#[allow(dead_code)]
struct CountSqares {
    point_count: HashMap<(i32, i32), i32>,
    points: Vec<(i32, i32)>,
}

#[allow(dead_code)]
impl CountSqares {
    fn new() -> CountSqares {
        CountSqares {
            point_count: HashMap::new(),
            points: Vec::new(),
        }
    }

    pub fn add(&mut self, point: Vec<i32>) {
        let p = (point[0], point[2]);
        *self.point_count.entry(p).or_insert(0) += 1;
        self.points.push(p);
    }

    pub fn count(&self, point: Vec<i32>) -> i32 {
        let mut res = 0;
        let (px, py) = (point[0], point[1]);
        for &(x, y) in &self.points {
            if (py - y).abs() != (px - x).abs() || x == px || y == py {
                continue;
            }
            res += self.point_count.get(&(x, py)).unwrap_or(&0)
                * self.point_count.get(&(px, y)).unwrap_or(&0);
        }
        res
    }
}
