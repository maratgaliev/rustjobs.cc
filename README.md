# rustjobs.cc

> Rust based API

## Setup

Install Rust first
```bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

Set the default toolchain
```bash

rustup default stable

```

Create database and setup Diesel
```bash

diesel setup

```

Run Diesel migrations
```bash

diesel migration run

```

Compile and run API (check .env file for settings)
```bash

cargo run

```

## Usage
  
```bash

curl -i -H "Accept: application/json" http://localhost:5000/jobs

```

And the result example

```bash
HTTP/1.1 200 OK
content-length: 357
content-type: application/json
date: Mon, 11 May 2020 21:15:22 GMT

[{"id":1,"title":"Rust developer","description":"Hiring experienced Rust developer","salary":100000,"currency":"USD","apply_url":"http://smartapps.ru","job_city":"Kazan","job_email":"contact@smartapps.ru","company":"SmartApps","company_twitter":"smartapps","company_website":"http://smartapps.ru","company_logo":null,"slug":"contact@mail.com"}]%
```

## License

[MIT](http://vjpr.mit-license.org)
