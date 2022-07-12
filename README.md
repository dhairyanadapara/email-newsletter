# Email Newsletter

## Prerequisites

### First Run

```
./scripts/init_db.sh
```

### DB migration

```
SKIP_DOCKER=true ./scripts/init_db.sh
```

### SQLX offline data

```dotnetcli
cargo sqlx prepare -- --lib
```
