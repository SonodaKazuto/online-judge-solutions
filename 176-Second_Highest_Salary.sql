-- LeetCode 176. Second Highest Salary
SELECT
    max(salary) as SecondHighestSalary
FROM
    Employee
WHERE
    salary not in (SELECT max(salary) FROM Employee)
;
