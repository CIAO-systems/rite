# Example calls
The dart demo server will work with the following example data
| Type | Value |
|------|-------|
| user (-u) | demo.user@ciao-systems.com |
| password (-p) | secret |
| API key (--api-key) | top-secret-api-key |


## Login
```bash
./login.sh \
    -s localhost:50051 \
    -u demo.user@ciao-systems.com \
    -p secret \
    --plaintext \
    --api-key=top-secret-api-key
```
## Clock
```bash
./clock.sh \
    -u demo.user@ciao-systems.com \
    -p secret \
    --time-type=f5553003-aece-4ef7-a64c-cd55d30f6fce \
    --plaintext \
    --api-key=top-secret-api-key
```