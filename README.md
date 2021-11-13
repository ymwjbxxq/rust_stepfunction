# Rust + SQS + Lambda + StepFunction

This is a straightforward example where the AWS Lambda function processes messages in parallel from an Amazon Simple Queue Service (Amazon SQS) queue and invokes AWS Step Functions.

You cannot find this example in the official [aws folder](https://github.com/awslabs/aws-sdk-rust/tree/main/examples) so I decided to write it up an example because AWS Step Functions is one of the most famous services. 

### TEST RESULTS ###

[Express StartExecution Quota](https://docs.aws.amazon.com/step-functions/latest/dg/limits-overview.html) are 6K
I have pushed 20K SQS 

* First case:  Add into DynamoDB
* Second case: Query DynamoDB and send it to a queue

I have tested both operations with 10K SQS messages:

* 128 MB
* 1024 MB

### 128 MB ###
Lambda:

![picture](https://github.com/ymwjbxxq/rust_stepfunction/blob/main/readme/128.png)

Express:

![picture](https://github.com/ymwjbxxq/rust_stepfunction/blob/main/readme/express-128-test.png)

### 1024 MB ###

Lambda:

![picture](https://github.com/ymwjbxxq/rust_stepfunction/blob/main/readme/1024.png)

Express:

![picture](https://github.com/ymwjbxxq/rust_stepfunction/blob/main/readme/express-1024-test.png)

### Build ###
```
make build
```

### Deploy ###
```
make deploy
```

### Cleanup ###
```
make delete
```