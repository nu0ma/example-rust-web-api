## Setup

- cd db
- ./build_and_run.sh

- cd app
- ./run.sh

## Test

- cargo test

## Example

```
❯ curl http://localhost:8085/v1/users/1
{"users":[{"id":1,"name":"test_user","organization_id":1}]}
```
