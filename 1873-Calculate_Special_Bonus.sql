-- LeetCode 1873. Calculate Special Bonus
SELECT
    employee_id,
    salary * (employee_id % 2) * (name not like 'M%') as bonus
FROM
    Employees
ORDER BY
    employee_id
;
