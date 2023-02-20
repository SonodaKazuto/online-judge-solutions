-- LeetCode 1084. Sales Analysis III
SELECT
    Product.product_id,
    product_name
FROM Product
LEFT JOIN Sales
ON Product.product_id = Sales.product_id
GROUP BY Sales.product_id
HAVING
    MIN(sale_date) >= '2019-01-01' and
    MAX(sale_date) <= '2019-03-31'
;
