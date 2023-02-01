-- LeetCode 627. Swap Salary
UPDATE
    Salary 
SET
    sex = case sex
          when 'm' then 'f'
          else 'm'
    end
;
