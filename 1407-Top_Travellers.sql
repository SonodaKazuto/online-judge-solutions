-- LeetCode 1407. Top Travellers
SELECT
    name,
    ifnull(sum(distance), 0) as travelled_distance
FROM Users
LEFT JOIN Rides
ON Users.id = Rides.user_id
GROUP BY name, user_id
ORDER BY travelled_distance desc, name
;
