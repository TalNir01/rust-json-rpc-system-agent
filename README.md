# rust-rest-api-agent

## Simple 'Agent' With REST-API
Here is the sample of API request:

## Examples

#### Raw HTTP Request
```http
POST /endpoint HTTP/1.1
Host: <target-ip>:3000
Content-Type: application/json
Content-Length: 109

{
    "status": "ExecCmd",
    "data": {
        "cmd": "ls -larth && 10",
        "timeout": 1
    }
}
```
#### cUrl Utility Request
```bash
curl --location '<target-ip>:3000/endpoint' \
--header 'Content-Type: application/json' \
--data '{
    "status": "ExecCmd",
    "data": {
        "cmd": "ls -larth && 10",
        "timeout": 1
    }
}'
```