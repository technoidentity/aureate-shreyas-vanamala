PRAGMA foreign_keys = ON;

SELECT s.name, c.title
FROM students AS s
INNER JOIN enrollments AS e ON s.id = e.student_id
INNER JOIN courses AS c ON e.course_id = c.id
ORDER BY s.id, c.id;

SELECT s.name, p.bio
FROM students AS s
LEFT JOIN student_profiles AS p ON s.id = p.student_id
ORDER BY s.id;

SELECT c.title, COUNT(e.student_id) AS total_students
FROM courses AS c
LEFT JOIN enrollments AS e ON c.id = e.course_id
GROUP BY c.id, c.title
ORDER BY c.id;


SELECT s.name, c.title
FROM students AS s
INNER JOIN enrollments AS e ON s.id = e.student_id
INNER JOIN courses AS c ON e.course_id = c.id
ORDER BY s.id, c.id;

SELECT s.name, c.title
FROM students AS s
INNER JOIN enrollments AS e ON s.id = e.student_id
INNER JOIN courses AS c ON e.course_id = c.id
WHERE c.title LIKE '%SQL';

INSERT INTO enrollments (student_id, course_id)
VALUES (1, 101);

INSERT INTO enrollments (student_id, course_id)
VALUES (4, 103);

SELECT s.name, c.title
FROM students AS s
INNER JOIN enrollments AS e ON s.id = e.student_id
INNER JOIN courses AS c ON e.course_id = c.id
WHERE s.id = 4;
