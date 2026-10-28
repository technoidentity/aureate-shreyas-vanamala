PRAGMA foreign_keys = ON;

SELECT id, name, email, age, status
FROM students
ORDER BY id;

INSERT INTO students (name, email, age)
VALUES ('Priya', 'priya@example.com', 21)
ON CONFLICT (email) DO NOTHING;

SELECT id, name, email, age, status
FROM students
WHERE email = 'priya@example.com';

CREATE TABLE IF NOT EXISTS student_notes (
    id INTEGER PRIMARY KEY,
    student_id INTEGER NOT NULL,
    note TEXT,
    FOREIGN KEY (student_id)
        REFERENCES students(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

PRAGMA foreign_key_list(student_notes);

PRAGMA foreign_keys;
