# AWS Rust Text Analysis Via SNS Post Processing

Based on [AWS | Text Analysis Via SNS Post Processing](https://www.serverless.com/examples/aws-node-text-analysis-via-sns-post-processing)

## Setup

```bash
$ npm install
```

## Run

```bash
$ npx serverless deploy

# Get the addNote endpoint and set to an environment variable
$ ENDPOINT=$(sls info | awk '{ if ($1 == "POST") { print $3 } }')

# Send request via API Gateway - replace ENDPOINT from `sls deploy` output
$ curl -X POST $ENDPOINT --data '{ "note": "it was the best of time" }'

# See results of analysis
$ npx serverless logs -f analyzeNote
```

## Cleanup

```bash
$ npx serverless remove
```

## Debug

### Add Note

```bash
$ npx serverless invoke -f addNote --path tests/add-note.json
```

### Analyze Note

```bash
$ npx serverless invoke -f analyzeNote --path tests/analyze-note.json
```