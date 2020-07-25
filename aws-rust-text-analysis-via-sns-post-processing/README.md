# aws-rust-text-analysis-via-sns-post-processing

Based on [AWS | Text Analysis Via SNS Post Processing](https://www.serverless.com/examples/aws-node-text-analysis-via-sns-post-processing)

## Setup

```bash
$ npm install
```

## Run

### Add Note

```bash
$ npx serverless invoke local -f add-note --path tests/add-note_success.json
```

### Analyze Note

```bash
$ npx serverless invoke local -f analyze-note --path [TODO]
```