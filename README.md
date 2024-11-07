# GitHub Trending API

## Introduction

GitHub Trending API is a self-hosted Rust microservice that converts GitHub trending repositories & developers HTML pages to JSON RPC like API.

## Project scope

1. There are no plans to provide cloud version of the service.

2. It provides as minimal information as possible.
For example, we intentionally didn't added stars count to repositories list,
because this data may be fetched directly from the GitHub REST/GraphQL APIs.

## Usage

### Get trending repository list

`POST /v1/listTrendingRepository`

```json
{
  "params": {
    "language": "rust"
  }
}
```

Response:
```json
{
  "error": null,
  "result": {
    "code": "SUCCESS",
    "list": [
      {
        "full_name": "rust-lang/rust",
        "rank": 1
      },
      {
        "full_name": "antonkomarev/github-trending-api",
        "rank": 2
      }
    ]
  }
}
```

### Get trending developer list

`POST /v1/listTrendingDeveloper`

```json
{
  "params": {
    "language": "rust"
  }
}
```

Response:
```json
{
  "error": null,
  "result": {
    "code": "SUCCESS",
    "list": [
      {
        "username": "ClementTsang",
        "rank": 1
      },
      {
        "username": "antonkomarev",
        "rank": 2
      }
    ]
  }
}
```

## License

- `GitHub Trending API` project is open-sourced software licensed under the [MIT license](LICENSE) by [Anton Komarev].

## About CyberCog

[CyberCog] is a Social Unity of enthusiasts. Research the best solutions in product & software development is our passion.

- [Follow us on Twitter](https://twitter.com/cybercog)

<a href="https://cybercog.su"><img src="https://cloud.githubusercontent.com/assets/1849174/18418932/e9edb390-7860-11e6-8a43-aa3fad524664.png" alt="CyberCog"></a>

[Anton Komarev]: https://komarev.com
[CyberCog]: https://cybercog.su
