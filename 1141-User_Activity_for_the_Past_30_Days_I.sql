-- LeetCode 1141. User Activity for the Past 30 Days I
SELECT activity_date as day, count(DISTINCT user_id) as active_users
FROM Activity
WHERE activity_date > '2019-06-27' and activity_date <= '2019-07-27'
GROUP BY activity_date
;
