-- LeetCode 197. Rising Temperature
SELECT DISTINCT today.id as id
FROM Weather today
INNER JOIN Weather other_day
WHERE datediff(today.recordDate, other_day.recordDate) = 1 and today.temperature > other_day.temperature
ORDER BY id
;
