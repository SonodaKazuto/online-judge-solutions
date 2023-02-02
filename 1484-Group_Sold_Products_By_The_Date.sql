-- LeetCode 1484. Group Sold Products By The Date
SELECT
    sell_date, 
    count(distinct product) as num_sold ,
    group_concat(distinct product order by product asc separator ',' ) as products
FROM 
    Activities group by sell_date 
ORDER BY
    sell_date asc
;
