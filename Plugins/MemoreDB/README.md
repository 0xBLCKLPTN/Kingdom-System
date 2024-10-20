# Memore

Memory-Cache service for `Kingdom-System` project.


## Usage
### Manually start server

```sh
user@machine %: iex -S mix
```

![alt text](https://github.com/0xBLCKLPTN/Memore/blob/master/docs/screenshots/code.png)

### Connect to server and manipulate with him
```sh
user@machine %:telnet 127.0.0.1 4040
```
![alt text](https://github.com/0xBLCKLPTN/Memore/blob/master/docs/screenshots/telnet_output.png)

### Commands:
```sh
GET, PUT, DELETE, CREATE
```

### Example:
```sh
user@machine %: telnet 127.0.0.1 4040
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.

CREATE shopping_list

PUT shopping_list milk 1
PUT shopping_list name 'Andrey'

GET shopping_list milk
GET shopping_list name

DELETE shopping_list milk
DELETE shopping_list name
```

### Dockerize:
```sh
docker build -t MemoreApplication .
docker run -dp 127.0.0.1:4040:4040 MemoreApplication
```


# Tests
All checks have now been passed
```sh
Finished in 0.1 seconds (0.06s async, 0.06s sync)
8 doctests, 2 tests, 1 failure
```

### TODO:
 - [x] ci/cd
 - [x] docker image
 - [x] gen_tcp server
 - [x] write docmodules
 - [x] write normal README.md 
