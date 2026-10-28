SELECT title FROM courses WHERE id IN (101, 102)
UNION
SELECT title FROM courses WHERE id IN (102, 103)
ORDER BY title;

SELECT title FROM courses WHERE id IN (101, 102)
UNION ALL
SELECT title FROM courses WHERE id IN (102, 103)
ORDER BY title;

SELECT title FROM courses WHERE id IN (101, 102)
INTERSECT
SELECT title FROM courses WHERE id IN (102, 103)
ORDER BY title;

SELECT title FROM courses WHERE id IN (101, 102)
EXCEPT
SELECT title FROM courses WHERE id IN (102, 103)
ORDER BY title;
