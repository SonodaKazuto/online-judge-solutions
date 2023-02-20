-- LeetCode 182. Duplicate Emails
SELECT DISTINCT person_1.email as Email
FROM Person person_1
INNER JOIN Person person_2
WHERE person_1.id < person_2.id and person_1.email = person_2.email
;
