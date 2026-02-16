# Database

## Status
Accepted

## Context
We need to store different kinds of data about books, so there needs to be a clear plan on how to store the data. 

## Decision
All data will be stored in 1NF, meaning

- atomic cells
- unique column names
- homogeneous data per column
- no repeating rows

One database will hold all tables because all the data is related and only one component of the app will need access to that data. If other components need data from the database, they will have to go through the database component's API.

## Data Model Overview

``` mermaid
erDiagram
    books ||--o{ books_series_link : has
    series ||--o{ books_series_link : has
    books ||--o{ books_authors_link : has
    authors ||--o{ books_authors_link : has
    read_books ||--o{ books : has
  series {
    integer id PK
    text name
    text sort
    integer goodreads_id UK
  }
  authors {
    integer id PK
    text name 
    text sort
    integer goodreads_id UK
  }

  books_authors_link {
    integer book FK
    integer author FK
  }

  books {
    integer id PK
    text title
    text sort
    timestamp date_added
    timestamp date_published
    timestamp date_modified
    integer page_count
    integer goodreads_id UK
  }


  read_books {
    integer id PK
    integer book FK
    timestamp start_date
    timestamp end_date
  }

  books_series_link {
    integer book FK
    integer series FK
    real volume
  }
```

Constraints and primary keys consisting of more than one column have been omitted here for brevity. 

## Consequences

### Positive

- Atomic columns make having multiple authors/series per book much easier to store
- Allows lots of constraints to be implemented directly on the database


### Negative

- Adding/Editing/Removing a book requires multiple `INSERT` queries because multiple tables have to be updated
- The data has to be transformed every time it crosses the boundaries between code and database due to the different shapes (multiple tables vs. one table with nested values)
