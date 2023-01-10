# VOXI core

## WIP warning

> **This project isn't finished and shouldn't be used in production yet.
> The API design can suffers break changes.**

## Objective

An Abstract Syntax Tree (AST) for building dynamic SQL queries.

Sintaxes available:

- Group by
- Sub-queries
- Joins
- Combinations
- Aggregated functions
- String functions
- Arithmetic expressions

Allowed types:

- i32 and i64
- boolean
- naive date
- naive datetime
- string
- uuid
- decimal

## Why can I need this?

You can serialize and deserialize the condition (or any other structure).

Arithmetic operators are defined for convenient operations with fields and literal values.

### Example

```rust
let id = TableField::new("PRODUCT.id");
let name = TableField::new("PRODUCT.name");
let price = TableField::new("PRODUCT.price");
let discount = TableField::new("PRODUCT.discount");

let log_expr = id
	.equal(1)
	.or(name.include(vec!["PS5", "XBOX"]))
	.exp()
	.and(price.less(1000 - discount));

let sql = args_to_str(log_expr).unwrap();
assert_eq!(
	sql,
	r#"("PRODUCT"."id" = 1 OR "PRODUCT"."name" IN ('PS5','XBOX')) AND "PRODUCT"."price" < 1000 - "PRODUCT"."discount""#
);

```

## SQLx

Can be paired with SQLx, when for all literal values will be defined arguments (binds), protecting against SQL injection. **(It will be released in a separated project voxi-dataset)**

SQLx is useful for constants queries, but sometimes you can need dynamic constructions of sources, fields or conditions.

### Example

```rust
// Generate query `SELECT "symbol" FROM "symbol" WHERE "symbol" = $1`
let query = QueryBuilder::new()
	.field("symbol")
	.from("symbol")
	.where_c(FieldName::new("symbol").equal("AAPL"))
	.build()
	.unwrap();

let executor = ExecutorSQLxPostgres::new(query);
let query_sqlx = executor.query().fetch_all(&pool);

let result = executor::block_on(query_sqlx)?;
for row in result.iter() {
	let id = row.try_get::<String, _>("symbol").unwrap();
	println!("{}", id);
}
```

## Current state and future changes

Some comments and unit test may be missing.

For now it provides support only syntax for Postgres.

Because it has many goals, probably this project needs to be divided in more projects.

- `voxi-sql`. AST for dynamic SQL.
- `voxi-dataset`. Types to manipulate relational database tables.

Many dependencies should be activated by features, like use of `uuid`, `rust_decimal`, `chrono` and `sqlx`.
