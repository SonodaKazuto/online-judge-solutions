impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        if bills.len() == 1 { return false; }

        let mut numbers_of_bills: (i32, i32) = (0, 0);
        for bill in bills.into_iter() {
            match bill {
                20 => {
                    if numbers_of_bills.1 == 0 {
                        if numbers_of_bills.0 >= 3 { numbers_of_bills.0 -= 3; }
                        else { return false; }
                    }
                    if numbers_of_bills.0 == 0 { return false; }
                    numbers_of_bills.1 -= 1;
                    numbers_of_bills.0 -= 1;
                }
                10 => {
                    if numbers_of_bills.0 == 0 { return false; }
                    numbers_of_bills.0 -= 1;
                    numbers_of_bills.1 += 1;
                }
                _ => numbers_of_bills.0 += 1
            }
        }

        return true;
    }
}
