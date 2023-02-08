-- LeetCode 1158. Market Analysis I
SELECT
    user_id as 'buyer_id',
    join_date,
    ifnull(orders_in_2019, 0) as orders_in_2019
FROM Users
LEFT JOIN (
    SELECT
        buyer_id,
        count(buyer_id) as orders_in_2019
    FROM Orders
    WHERE year(order_date) = 2019
    GROUP BY buyer_id
) as Orders
ON Users.user_id = Orders.buyer_id
;
