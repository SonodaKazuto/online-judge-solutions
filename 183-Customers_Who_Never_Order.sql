-- LeetCode 183. Customers Who Never Order
SELECT
    name as 'Customers'
FROM
    Customers
WHERE
    id not in (SELECT customerId FROM Orders)
;
