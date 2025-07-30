# KubeSend
[![Docker Pulls](https://img.shields.io/docker/pulls/v3xl/kubesend.svg)](https://hub.docker.com/r/v3xl/kubesend/) [![Artifact Hub](https://img.shields.io/endpoint?url=https://artifacthub.io/badge/repository/kubesend)](https://artifacthub.io/packages/search?repo=kubesend)

A flexible notification service for Kubernetes environments.

## Overview

KubeSend provides a simple way to send notifications from your Kubernetes environment through various channels. It's designed to be integrated with your existing services.

This was made mainly for the purpose of learning Rust. If this is a project which interests you too, feel free to contribute and improve upon it.

## Features

### Implemented 

- **Notification Channels**
  - [x] SMTP (Email)

### Coming Soon 

- **Additional Notification Channels**
  - [ ] Telegram
  - [ ] Discord
- **Enhanced Functionality**
  - [ ] Message templates
  - [ ] Message queuing
  - [ ] Message retries

Note: The following features are not included:
- Cronjobs and scheduling tasks

## Install with Helm

Add the KubeSend repository.
```bash
# Add the KubeSend Helm repository
helm repo add kubesend https://v3xl.github.io/KubeSend/

# Update your Helm repositories
helm repo update
```


Modify the values.yaml according to your requirements, most importantly specifying your SMTP settings.

```bash
helm show values kubesend/kubesend > values.yaml
```


Install KubeSend using your modified values
```bash
helm install kubesend kubesend/kubesend -f values.yaml
```




## Docker Install (For basic testing)

Create the config.yaml file (see config.yaml.example), then run with Docker for testing.
```bash
docker run --rm -it -p 8080:8080 -v /path/to/config:/app/config v3xl/kubesend
```

## Usage

### Send an Email
The below shows a CURL example.
```bash
curl -X POST http://kubesender/smtp/send \
  -H "Content-Type: application/json" \
  -d '{
    "to": [
      { "email": "example.recipient@example.com" }
    ],
    "from": {
      "name": "ExampleSender",
      "email": "noreply@example.com"
    },
    "subject": "Test Email Subject",
    "html_body": "<p>Hello, this is a <strong>test</strong> email sent via KubeSender.</p>",
    "text_body": "Hello, this is a test email sent via KubeSender."
  }'
```
