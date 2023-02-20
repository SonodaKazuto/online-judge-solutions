-- LeetCode 1587. Bank Account Summary II
SELECT
    name,
    sum(amount) as 'balance'
FROM Users
LEFT JOIN Transactions
ON Users.account = Transactions.account
GROUP BY Transactions.account
HAVING balance > 10000
;
