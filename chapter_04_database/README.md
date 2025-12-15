# Chapter 4: Database & Storage

## Learning Objectives

After completing this chapter, you will be able to:

1. **SQL & Database Access**
   - Use SQLx for type-safe database queries
   - Implement CRUD operations with PostgreSQL/SQLite
   - Understand connection pooling and its importance
   - Handle database migrations

2. **Caching**
   - Understand caching strategies (read-through, write-through, cache-aside)
   - Use Redis for distributed caching
   - Implement cache invalidation patterns
   - Handle cache failures gracefully

## Chapter Structure

```
chapter_04_database/
├── README.md                    # This file
├── checkpoint.md                # Self-assessment
├── 01_sql/
│   ├── theory.md               # SQL, SQLx, connection pooling
│   ├── lab_01_sqlx_crud/       # Basic CRUD with SQLx
│   └── lab_02_connection_pool/ # Connection pool implementation
└── 02_caching/
    ├── theory.md               # Caching strategies, Redis
    ├── lab_03_redis_basics/    # Redis operations
    └── lab_04_cache_patterns/  # Cache-aside pattern
```

## Prerequisites

- Completed Chapter 3 (HTTP and REST APIs)
- Basic SQL knowledge
- Docker (for running PostgreSQL and Redis)

## Setup Requirements

### PostgreSQL (for Labs 1-2)
```bash
# Using Docker
docker run -d \
  --name postgres-lab \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=testdb \
  -p 5432:5432 \
  postgres:15

# Or use SQLite (no setup needed)
```

### Redis (for Labs 3-4)
```bash
# Using Docker
docker run -d \
  --name redis-lab \
  -p 6379:6379 \
  redis:7
```

## Labs Overview

| Lab | Topic | Key Concepts |
|-----|-------|--------------|
| Lab 1 | SQLx CRUD | Async queries, type safety, migrations |
| Lab 2 | Connection Pool | Pool sizing, timeouts, health checks |
| Lab 3 | Redis Basics | GET/SET, TTL, data structures |
| Lab 4 | Cache Patterns | Cache-aside, invalidation, fallback |

## Tools for Observation

```bash
# PostgreSQL
psql -h localhost -U postgres -d testdb
\dt                              # List tables
\d table_name                    # Describe table
EXPLAIN ANALYZE SELECT ...;      # Query plan

# Redis
redis-cli
KEYS *                           # List all keys
GET key                          # Get value
TTL key                          # Time to live
INFO stats                       # Server stats

# Connection monitoring
ss -tlnp | grep 5432             # PostgreSQL connections
ss -tlnp | grep 6379             # Redis connections
```

## Recommended Reading

- SQLx documentation: https://docs.rs/sqlx
- Redis documentation: https://redis.io/docs
- "Designing Data-Intensive Applications" by Martin Kleppmann

## Time Estimate

- Theory reading: 2-3 hours
- Labs: 4-6 hours
- Total: 6-9 hours
