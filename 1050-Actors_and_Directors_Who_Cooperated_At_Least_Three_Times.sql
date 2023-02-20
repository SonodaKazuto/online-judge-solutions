-- LeetCode 1050. Actors and Directors Who Cooperated At Least Three Times
SELECT
        actor_id,
        director_id
FROM (
    SELECT
            actor_id,
            director_id,
            count(director_id) as director_count
    FROM ActorDirector
    GROUP BY actor_id, director_id
) AD_count
WHERE director_count >= 3
;
