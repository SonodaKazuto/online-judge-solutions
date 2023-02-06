-- LeetCode 607. Sales Person
SELECT name
FROM SalesPerson
WHERE sales_id not in (
    SELECT sales_id
    FROM Orders
    LEFT JOIN Company
    ON Orders.com_id = Company.com_id
    WHERE name = 'RED'
)
;
