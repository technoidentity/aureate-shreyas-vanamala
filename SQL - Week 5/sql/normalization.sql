PRAGMA foreign_keys = ON;

SELECT student_id, student_name, course_id, course_title,
       instructor_id, instructor_name
FROM enrollments_flat
ORDER BY student_id, course_id;

SELECT course_id, course_title, COUNT(*) AS repeated_rows
FROM enrollments_flat
GROUP BY course_id, course_title
ORDER BY course_id;

SELECT id, name FROM students ORDER BY id;
SELECT student_id, course_id FROM enrollments ORDER BY student_id, course_id;
SELECT id, title, instructor_id, instructor_name FROM courses_2nf ORDER BY id;

SELECT id, name FROM instructors ORDER BY id;
SELECT id, title, instructor_id FROM courses ORDER BY id;

SELECT s.name AS student, c.title AS course, i.name AS instructor
FROM enrollments AS e
INNER JOIN students AS s ON e.student_id = s.id
INNER JOIN courses AS c ON e.course_id = c.id
INNER JOIN instructors AS i ON c.instructor_id = i.id
ORDER BY s.id, c.id;


SELECT * FROM students;

SELECT * FROM courses_2nf;

SELECT * FROM enrollments;

SELECT * FROM instructors;

SELECT * FROM courses;
