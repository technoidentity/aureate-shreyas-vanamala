  CREATE TABLE children(
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        age INTEGER
    );


INSERT INTO students (id, name, age)
VALUES
    (1, 'Arshad', 23),
    (2, 'Shreyas', 22),
    (3, 'Alex', 24);

INSERT INTO students (id, name, age)
VALUES
    (4, 'Priya', 21),
    (5, 'Rahul', 25),
    (6, 'Sneha', 22),
    (7, 'Kiran', 24),
    (8, 'Ananya', 20),
    (9, 'Vikram', 23),
    (10, 'Meera', 26);

SELECT *
FROM students
WHERE age > 22;

SELECT name, age
FROM students
WHERE name = 'Shreyas';


SELECT name, age
from students
where age  >23;


select name, age
from students
where  age = 24 AND name = 'Alex';

select * from students ORDER BY age DESC;

select * from students ORDER BY age ASC LIMIT 1;

SELECT name, age
FROM students
WHERE age >= 23
ORDER BY age DESC
LIMIT 2;

SELECT name, age
FROM students
Where age >= 23
ORDER BY age DESC
LIMIT 2;

Select * FROM students where name like 'S%';

select * FROM students where name like '%s';

select * FROM students where age IN(22,24);

select * from students where name NOT IN('Shreyas', 'Arshad');

    SELECT * FROM students
    where age BETWEEN 22 and 24 ORDER BY age ASC;

select name , age
from students
where age > 22 AND name like 'A%';

SELECT name, age
FROM students
WHERE age >= 23 AND age <= 24
ORDER BY age ASC;

SELECT
    COUNT(*) AS total_students,
    MIN(age) AS youngest,
    MAX(age) AS oldest,
    AVG(age) AS average_age,
    SUM(age) AS total_age
FROM students;

select count(*) as adult_students FROM students where age >= 18;

SELECT * FROM students;

select *from students where age = (select AVG(age) from students);

select AVG(age) as average_age from students;

SELECT AVG(age) AS average_age
FROM students
WHERE age >= 23;


SELECT age, COUNT(*) AS total_students
FROM students
GROUP BY age
HAVING COUNT(*) = 1;

INSERT into students (id , name , age) values (11 , 'Sai', NULL);


SELECT *
FROM students
WHERE age IS NULL;

SELECT * FROM students
where age IS NOT NULL;

SELECT
    COUNT(*) AS total_students,
    COUNT(age) AS students_with_age,
    AVG(age) AS average_age
FROM students;


SELECT name, age from students where age is NULL;

UPDATE students
SET age = 25
WHERE id = 6;


SELECT * FROM students WHERE id = 6;

INSERT INTO students (id, name, age)
VALUES (99, 'Temporary', 20);

DELETE FROM students
WHERE id = 99;

UPDATE students SET age = 23 WHERE id = 5;

Select *from students where age = 23;