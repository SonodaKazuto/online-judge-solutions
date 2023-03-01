// LeetCode 1491. Average Salary Excluding the Minimum and Maximum Salary
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut sorted_salary: Vec<i32> = salary.clone();

        sorted_salary.sort();

        let mut sum_salary: i32 = sorted_salary.iter().sum();
        sum_salary -= (*sorted_salary.first().unwrap() + *sorted_salary.last().unwrap());

        return (sum_salary as f64) / (sorted_salary.len() as f64 - 2.0);
    }
}
