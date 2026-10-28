PRAGMA foreign_keys = ON;

SELECT id, name, phone_numbers
FROM students_unstructured
ORDER BY id;

UPDATE students
SET name = 'Shreyas Vanamala'
WHERE id = 2;

SELECT id, name FROM students ORDER BY id;
SELECT id, student_id, phone_number FROM student_phones ORDER BY id;

SELECT s.name, p.phone_number
FROM students AS s
INNER JOIN student_phones AS p
    ON s.id = p.student_id
WHERE s.id = 2;
