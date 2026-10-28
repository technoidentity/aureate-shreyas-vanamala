CREATE VIEW IF NOT EXISTS student_courses AS
SELECT s.id AS student_id, s.name AS student_name, c.title AS course
FROM students AS s
INNER JOIN enrollments AS e ON s.id = e.student_id
INNER JOIN courses AS c ON e.course_id = c.id;

SELECT student_name, course
FROM student_courses
ORDER BY student_id, course;

SELECT student_name, course
FROM student_courses
WHERE course = 'SQL'
ORDER BY student_id;
