-- LeetCode 196. Delete Duplicate Emails
DELETE 
    user_1 
FROM 
    Person as user_1  
INNER JOIN 
    Person as user_2   
WHERE 
    user_1.id > user_2.id AND user_1.email = user_2.email
;  
