-- LeetCode 1393. Capital Gain/Loss
SELECT
    stock_name,
    sum(
        case when operation = 'Sell' 
            then price 
            else -price
        end
        ) as capital_gain_loss
FROM Stocks
GROUP BY stock_name
;
