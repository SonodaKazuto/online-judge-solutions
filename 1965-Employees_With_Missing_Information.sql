-- LeetCode 1965. Employees With Missing Information
SELECT employee_id 
FROM Employees 
WHERE employee_id not in (SELECT employee_id FROM Salaries)
UNION
SELECT employee_id 
FROM Salaries
WHERE employee_id not in (SELECT employee_id FROM Employees)
ORDER BY
    employee_id
;
