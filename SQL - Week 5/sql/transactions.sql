SELECT id, name FROM students ORDER BY id;

BEGIN;

UPDATE students SET name = 'Arshad (practice)' WHERE id = 1;
UPDATE students SET name = 'Alex (practice)' WHERE id = 3;

SELECT id, name FROM students ORDER BY id;

ROLLBACK;

SELECT id, name FROM students ORDER BY id;

BEGIN;

UPDATE students SET name = 'Shreyas Vanamala' WHERE id = 2;

COMMIT;

SELECT id, name FROM students WHERE id = 2;
