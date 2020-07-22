# rust-lambda

Rust Lambda implementation of Serverless [aws-nodejs](https://github.com/serverless/serverless/tree/fc8c864c3b6d8e74137b3c42d5799ea105d4bac7/lib/plugins/create/templates/aws-nodejs) template.

## Dev Dependencies

* [Serverless Components](https://github.com/serverless/components)

## Setup

```bash
$ npm install
```

## Run

```bash
$ npx serverless invoke local -f hello --path test/event.json
```
