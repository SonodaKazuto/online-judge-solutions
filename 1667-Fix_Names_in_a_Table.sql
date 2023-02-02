-- LeetCode 1667. Fix Names in a Table
SELECT 
    user_id, concat(ucase(left(name, 1)), lcase(substring(name, 2))) as name
FROM
    Users
ORDER BY
    user_id
;
