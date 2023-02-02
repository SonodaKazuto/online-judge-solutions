-- LeetCode 1527. Patients With a Condition
SELECT
    *
FROM
    Patients
WHERE
    conditions like 'DIAB1%' or conditions like '% DIAB1%'
;
