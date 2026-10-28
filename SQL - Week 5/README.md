# SQL Practice — Week 5

SQL exercises from my database learning track, using SQLite. The examples use students, courses, enrollments, phone numbers, and employee–manager relationships.

## Practice files

| File | Topics | Local database |
| --- | --- | --- |
| [table_creation.sql](sql/table_creation.sql) | Creating a table, column types, primary keys | `learning.db` |
| [sql_basics.sql](sql/sql_basics.sql) | CRUD, filtering, sorting, aggregates, grouping, and NULL | `learning.db` |
| [table_design.sql](sql/table_design.sql) | Atomic values, separate phone records, and one-to-many relationships | `table_design.db` |
| [joins_and_subqueries.sql](sql/joins_and_subqueries.sql) | INNER/LEFT JOIN, aliases, COALESCE, and basic subqueries | `relationships_and_joins.db` |
| [relationships_and_composite_keys.sql](sql/relationships_and_composite_keys.sql) | One-to-one, many-to-many, junction tables, and composite keys | `relationship_types.db` |
| [normalization.sql](sql/normalization.sql) | Comparing repeated data with separated student, course, and instructor tables | `normalization.db` |
| [constraints.sql](sql/constraints.sql) | Defaults, foreign-key definitions, and cascade rules | `constraints.db` |
| [constraint_failures.sql](sql/constraint_failures.sql) | CHECK, UNIQUE, and NOT NULL rejection examples | `constraints.db` |
| [views.sql](sql/views.sql) | Creating and filtering a view over joined tables | `normalization.db` |
| [transactions.sql](sql/transactions.sql) | BEGIN, ROLLBACK, and COMMIT | `normalization.db` |
| [set_operations.sql](sql/set_operations.sql) | UNION, UNION ALL, INTERSECT, and EXCEPT | `normalization.db` |
| [self_joins.sql](sql/self_joins.sql) | Self-referencing foreign keys and employee–manager queries | `normalization.db` |

## Running the exercises

- Use SQLite and select the matching database before executing a query. Opening a different SQL file does not switch the connection.
- These are practice statements, not a single migration. Run individual statements or examples. Repeating CREATE or INSERT statements can produce already-exists or duplicate-key errors.
- The database files and separate setup scripts stay local and are excluded from Git. The matching schemas and sample data are required to run these exercises.
- `constraint_failures.sql` is intentionally invalid against the data rules. Run each statement separately and inspect the error. The older JOIN and relationship files also contain deliberate foreign-key and duplicate-enrollment tests.

For foreign-key and transaction practice, use one persistent SQLite terminal connection. From this folder, with the existing local databases available:

```bash
cd sql
sqlite3 normalization.db
```

At the SQLite prompt:

```sql
PRAGMA foreign_keys = ON;
PRAGMA foreign_keys;
```

The second statement should return `1`. Enable enforcement outside a transaction. Run the transaction example with:

```text
.read transactions.sql
```

The SQL Notebook extension used during practice resets SQLite connection settings while saving and executes only the first statement in a cell. Use the terminal for transaction and foreign-key exercises.

## References

- *Head First SQL* — Lynn Beighley, with a focus on chapters 4 and 7–11.
- *Grokking Relational Database Design* — Qiang Hao and Michail Tsikerdekis, companion examples.
