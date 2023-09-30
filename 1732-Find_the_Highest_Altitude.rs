impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        return *gain.into_iter()
                    .fold(vec![0], |mut route, d| {
                        route.push(route.last().unwrap() + d);
                        route
                    })
                    .iter()
                    .max()
                    .unwrap();
    }
}
