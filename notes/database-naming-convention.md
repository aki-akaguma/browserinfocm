# Database Naming Conventions

Database naming conventions provide consistent rules for naming tables, columns, and other objects, promoting readability and maintainability, typically using lowercase, `snake_case` (underscores for spaces), descriptive nouns/adjectives, avoiding reserved words/spaces/special characters, and choosing either singular or plural for tables consistently (e.g., users or user, but not both). Key practices include full words (not abbreviations), specific patterns for IDs (`item_id`), and clear foreign key naming (`source_warehouse_id`)


## General Rules

- Case: Use lowercase for everything (tables, columns, views).
- Separators: Use underscores (`_`) for multiple words (`snake_case`), e.g., `product_name`, not `ProductName` or `productname`.
- No Spaces/Special Chars: Avoid spaces, dashes, or dots in names.
- Full Words: Use `payment_date` instead of `pmnt_dt` for clarity.
- Descriptive: Names should clearly describe the data they hold


## Tables

- Consistency: Choose the plural form (e.g., `customers`) instead of the singular form (`customer`) and stick with it.
- Meaning: Use nouns that represent the entity (e.g., `orders`, `products`).
- Associative Tables: For many-to-many joins, `table1_table2` (e.g., `items_orders`)


## Columns

- Singular: Generally use singular nouns for columns (e.g., `first_name`, `order_date`).
- IDs: Use `singular_noun_id` (e.g., `user_id`, `product_id`).
- Foreign Keys (FKs): Name FKs the same as the primary key they reference (e.g., `warehouse_id`), potentially with context like `source_warehouse_id`


## Specific Object Types

- Primary Keys: Often id or `table_name_id`.
- Foreign Keys: `referenced_table_id` (e.g., `customer_id` in an orders table).
- Timestamps: `created_at`, `updated_at`.
- Dates: `arrival_on`, `sales_on`.

