-- LeetCode 608. Tree Node
SELECT
    id, 'Root' as type
FROM
    Tree
WHERE
    p_id is NULL
UNION
SELECT
    id, 'Inner' as type
FROM
    Tree
WHERE
    p_id in (SELECT DISTINCT id FROM Tree) and 
    id in (SELECT DISTINCT p_id FROM Tree WHERE p_id is not NULL)
UNION
SELECT
    id, 'Leaf' as type
FROM
    Tree
WHERE
    p_id in (SELECT DISTINCT id FROM Tree) and 
    id not in (SELECT DISTINCT p_id FROM Tree WHERE p_id is not NULL)
;
