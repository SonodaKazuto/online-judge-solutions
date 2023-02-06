-- LeetCode 1729. Find Followers Count
SELECT user_id, count(DISTINCT follower_id) as followers_count
FROM Followers
GROUP BY user_id
ORDER BY user_id asc
;
