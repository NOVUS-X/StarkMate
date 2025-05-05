## Database Indexing

Indexes are crucial for optimizing database query performance. They allow the database engine to find rows matching WHERE clauses and ORDER BY clauses more quickly, avoiding costly full-table scans.

### When to Add Indexes

- **Frequently Queried Columns:** Add indexes to columns frequently used in `WHERE` clauses (e.g., `player_id`, `game_id`).
- **Ordering Columns:** Index columns used in `ORDER BY` clauses, especially if the query involves large datasets (e.g., `created_at` for recent games).
- **Foreign Keys:** Although PostgreSQL automatically indexes primary keys, foreign keys often benefit from explicit indexes to speed up joins.

### Choosing the Right Index Type

- **B-Tree (Default):** Best for equality (`=`) and range (`<`, `>`, `BETWEEN`) queries. Suitable for most common use cases, including `player_id`, `id`, and `created_at`.
- **Hash:** Only useful for simple equality comparisons (`=`). Generally less flexible than B-Tree.
- **GIN/GIST:** Used for indexing complex data types like arrays, full-text search data, or geometric data.

### Best Practices

- **Analyze Query Patterns:** Before adding an index, use `EXPLAIN ANALYZE` on your typical queries to see if an index would actually help. Sometimes, the query planner might opt for a sequential scan if the table is small or the query returns a large portion of the table.
- **Avoid Over-Indexing:** Each index adds overhead to write operations (`INSERT`, `UPDATE`, `DELETE`) and consumes storage space. Only index columns that provide significant query performance benefits.
- **Composite Indexes:** If queries frequently filter or order by multiple columns, consider a composite index (e.g., `CREATE INDEX idx_some_table_col1_col2 ON some_table(col1, col2)`). The order of columns matters.
- **Index Naming Convention:** Use a consistent naming convention, such as `idx_<table_name>_<column_names>`.
- **Conditional Creation:** Use `IF NOT EXISTS` when creating indexes in migrations to prevent errors if the migration is run multiple times.
- **Maintenance:** Run `VACUUM` and `ANALYZE` regularly (often handled automatically by PostgreSQL autovacuum) to keep table statistics up-to-date, ensuring the query planner uses indexes effectively.
- **Monitor Index Usage:** Periodically check for unused indexes using PostgreSQL's statistics views (e.g., `pg_stat_user_indexes`) and drop those that are no longer beneficial.
- **Concurrent Creation:** For large tables in production, create indexes using the `CONCURRENTLY` option (`CREATE INDEX CONCURRENTLY ...`) to avoid locking the table during index creation.
