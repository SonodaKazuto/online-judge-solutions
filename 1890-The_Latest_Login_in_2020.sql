-- LeetCode 1890. The Latest Login in 2020
SELECT
    user_id,
    max(time_stamp) as last_stamp
FROM Logins
WHERE year(time_stamp) = 2020
GROUP BY user_id
;
