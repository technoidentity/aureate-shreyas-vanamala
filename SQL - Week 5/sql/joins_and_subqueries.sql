
SELECT students.name, students.age, courses.title
FROM students
INNER JOIN courses
    ON students.course_id = courses.id;





SELECT students.name, students.age, courses.title
FROM students
INNER JOIN courses
    ON students.course_id = courses.id
WHERE courses.title = 'Rust';

SELECT students.name, students.age, courses.title
FROM students
INNER JOIN courses
    ON students.course_id = courses.id
WHERE courses.title = 'Rust' AND students.age > 20;

SELECT students.name, students.age, courses.title
FROM students
INNER JOIN courses
    ON students.course_id = courses.id
WHERE courses.title = 'Rust' AND students.age > 23;

SELECT students.name, students.age, courses.title
FROM students
LEFT Join courses
    on students.course_id = courses.id
    WHERE courses.id IS NULL;

SELECT students.name, students.age, courses.title
FROM students
LEFT JOIN courses
    ON students.course_id = courses.id
WHERE courses.id IS NULL;

SELECT courses.title, students.name
FROM courses
LEFT JOIN students
    ON courses.id = students.course_id
    WHERE students.id IS NULL;



SELECT courses.title, COUNT(students.id) AS total_students
FROM courses
LEFT JOIN students
    ON courses.id = students.course_id
GROUP BY courses.id, courses.title
HAVING COUNT(students.id) >= 2;

SELECT s.name, c.title
FROM students AS s
LEFT JOIN courses AS c
    ON s.course_id = c.id;

SELECT
    s.name,
    COALESCE(c.title, 'Not assigned') AS course
FROM students AS s
LEFT JOIN courses AS c
    ON s.course_id = c.id;

PRAGMA foreign_keys = ON;

UPDATE students
SET Course_id = 999
WHERE id = 1;

UPDATE students SET course_id =102
where id = 4 ;

SELECT s.name, c.title
FROM students AS s
LEFT JOIN courses AS c
on s.course_id = c.id
WHERE s.id = 4;

DELETE FROM courses
WHERE id = 102;

SELECT * FROM courses WHERE id = 102;

INSERT INTO courses (id, title)
VALUES (102, 'SQL');

PRAGMA foreign_keys;

/*markdown
SUBQUERIES
*/



SELECT name,age
FROM students
WHERE age <( SELECT AVG(age) FROM students );


SELECT name, age
FROM students
WHERE course_id IN (SELECT id FROM courses WHERE title IN('Rust', 'SQL'));
