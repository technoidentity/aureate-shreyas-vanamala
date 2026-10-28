CREATE TABLE IF NOT EXISTS employees (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    manager_id INTEGER REFERENCES employees(id)
);

INSERT INTO employees (id, name, manager_id)
VALUES
    (1, 'Arshad', NULL),
    (2, 'Shreyas', 1),
    (3, 'Alex', 1),
    (4, 'Priya', 2)
ON CONFLICT (id) DO NOTHING;

SELECT e.name AS employee, m.name AS manager
FROM employees AS e
LEFT JOIN employees AS m ON e.manager_id = m.id
ORDER BY e.id;

SELECT e.name AS employee, m.name AS manager
FROM employees AS e
INNER JOIN employees AS m ON e.manager_id = m.id
ORDER BY e.id;
