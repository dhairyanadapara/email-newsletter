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
cargo sqlx prepare --merged -- --all-targets --all-features
```

### Build docker image

```
docker build --tag email-newsletter --file Dockerfile .
```

### Deploy to Heroku

```
heroku login

docker login --username=<email_add_used_to_signup_to_heroku> password=$(heroku auth:token) registry.heroku.com

heroku container:login

docker images

docker tag email-newsletter registry.heroku.com/rust-email-newsletter/web

heroku container:push web -a rust-email-newsletter

heroku container:release web -a rust-email-newsletter

```

url: <https://rust-email-newsletter.herokuapp.com>
