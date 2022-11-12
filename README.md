# Simple Rust Web API Example

## Setup

```
# run db
cd db
./build_and_run.sh

# run app
cd app
./run.sh
```

## Test

`cargo test`

## Example

```
❯ curl http://localhost:8085/v1/users/1
{"users":[{"id":1,"name":"test_user","organization_id":1}]}
```
