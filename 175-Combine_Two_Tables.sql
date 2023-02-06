-- LeetCode 175. Combine Two Tables
SELECT lastName, firstName, city, state
FROM Person
LEFT JOIN Address
ON Person.personId = Address.personId
;
