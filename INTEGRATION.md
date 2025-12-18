# 🔗 TRAE CLI - Integration Guide

## JARVIXSERVER Integration

TRAE CLI integrates seamlessly with JARVIXSERVER to provide comprehensive code analysis and repair capabilities through a unified API.

### Architecture

```
┌─────────────────┐    ┌─────────────────┐
│   JARVIXSERVER  │    │    TRAE CLI     │
│    (Port 8080)  │◄──►│   (Port 3001)   │
│                 │    │                 │
│ • API Gateway   │    │ • Code Analysis │
│ • Proxy Router  │    │ • Auto Repair   │
│ • Metrics Hub   │    │ • Quality Score │
└─────────────────┘    └─────────────────┘
```

### Proxy Configuration

JARVIXSERVER automatically proxies TRAE CLI endpoints under `/trae/*`:

```javascript
// JARVIXSERVER proxy routes
GET  /trae/health     → http://localhost:3001/health
POST /trae/api/analyze → http://localhost:3001/api/analyze
POST /trae/api/repair → http://localhost:3001/api/repair
GET  /trae/api/metrics → http://localhost:3001/api/metrics
```

### Health Check Integration

```bash
# Direct TRAE CLI health check
curl http://localhost:3001/health

# Via JARVIXSERVER proxy
curl http://localhost:8080/trae/health
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Code Quality Analysis
on: [push, pull_request]

jobs:
  analysis:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build TRAE CLI
        run: cargo build --release --bin trae-server

      - name: Start TRAE Server
        run: |
          ./target/release/trae-server &
          sleep 3

      - name: Run Code Analysis
        run: |
          ANALYSIS=$(curl -X POST http://localhost:3001/api/analyze)
          echo "Analysis Results: $ANALYSIS"

          # Check quality score
          SCORE=$(echo $ANALYSIS | jq '.data.quality_score')
          if (( $(echo "$SCORE < 70" | bc -l) )); then
            echo "❌ Quality score too low: $SCORE"
            exit 1
          fi

      - name: Auto Repair
        run: curl -X POST http://localhost:3001/api/repair

      - name: Generate Report
        run: |
          METRICS=$(curl http://localhost:3001/api/metrics)
          echo "## Code Quality Report" >> $GITHUB_STEP_SUMMARY
          echo "$METRICS" | jq -r '.data | to_entries[] | "- **\(.key)**: \(.value)"' >> $GITHUB_STEP_SUMMARY
```

### Jenkins Pipeline

```groovy
pipeline {
    agent any

    stages {
        stage('Code Analysis') {
            steps {
                sh '''
                    # Build TRAE CLI
                    cargo build --release --bin trae-server

                    # Start server in background
                    ./target/release/trae-server &
                    sleep 3

                    # Run analysis
                    curl -X POST http://localhost:3001/api/analyze > analysis.json

                    # Check results
                    SCORE=$(jq '.data.quality_score' analysis.json)
                    if (( $(echo "$SCORE < 70" | bc -l) )); then
                        echo "Quality score too low: $SCORE"
                        exit 1
                    fi
                '''
            }
        }
    }

    post {
        always {
            archiveArtifacts artifacts: 'analysis.json', allowEmptyArchive: true
        }
    }
}
```

### Docker Integration

```dockerfile
# Dockerfile for TRAE CLI with Chapel support
FROM rust:1.70-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin trae-server

FROM debian:bookworm-slim
# Install dependencies for Chapel
RUN apt-get update && apt-get install -y \
    wget \
    tar \
    gcc \
    g++ \
    make \
    python3 \
    python3-dev \
    llvm \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Download and install Chapel
RUN wget -O chapel.tar.gz https://github.com/chapel-lang/chapel/releases/download/1.30.0/chapel-1.30.0.tar.gz \
    && tar -xzf chapel.tar.gz \
    && cd chapel-1.30.0 \
    && make \
    && make install \
    && cd .. \
    && rm -rf chapel-1.30.0 chapel.tar.gz

# Set Chapel environment
ENV CHPL_HOME=/usr/local/chapel
ENV PATH=$PATH:$CHPL_HOME/bin

COPY --from=builder /app/target/release/trae-server /usr/local/bin/
EXPOSE 3001
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3001/health || exit 1
CMD ["trae-server"]
```

```yaml
# docker-compose.yml
version: '3.8'
services:
  trae-cli:
    build: .
    ports:
      - "3001:3001"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3001/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  jarvixserver:
    image: jarvixserver:latest
    ports:
      - "8080:8080"
    depends_on:
      trae-cli:
        condition: service_healthy
```

## API Integration

### REST API Usage

```python
import requests

class TraeClient:
    def __init__(self, base_url="http://localhost:3001"):
        self.base_url = base_url

    def health_check(self):
        return requests.get(f"{self.base_url}/health").json()

    def analyze_project(self, path="."):
        return requests.post(f"{self.base_url}/api/analyze", json={"path": path}).json()

    def auto_repair(self):
        return requests.post(f"{self.base_url}/api/repair").json()

    def get_metrics(self):
        return requests.get(f"{self.base_url}/api/metrics").json()

# Usage
client = TraeClient()
analysis = client.analyze_project()
print(f"Quality Score: {analysis['data']['quality_score']}")
```

### JavaScript/Node.js

```javascript
const axios = require('axios');

class TraeAPI {
    constructor(baseURL = 'http://localhost:3001') {
        this.client = axios.create({ baseURL });
    }

    async analyze() {
        const response = await this.client.post('/api/analyze');
        return response.data;
    }

    async repair() {
        const response = await this.client.post('/api/repair');
        return response.data;
    }

    async getMetrics() {
        const response = await this.client.get('/api/metrics');
        return response.data;
    }
}

// Usage
const trae = new TraeAPI();
const analysis = await trae.analyze();
console.log(`Issues found: ${analysis.data.issues.length}`);
```

## Monitoring Integration

### Prometheus Metrics

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'trae-cli'
    static_configs:
      - targets: ['localhost:3001']
    metrics_path: '/api/metrics'
    scrape_interval: 15s
```

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "TRAE CLI Metrics",
    "panels": [
      {
        "title": "Quality Score",
        "type": "gauge",
        "targets": [
          {
            "expr": "trae_quality_score",
            "legendFormat": "Quality Score"
          }
        ]
      },
      {
        "title": "Issues Over Time",
        "type": "graph",
        "targets": [
          {
            "expr": "trae_issues_total",
            "legendFormat": "Total Issues"
          }
        ]
      }
    ]
  }
}
```

## Webhook Integration

### Slack Notifications

```bash
#!/bin/bash
# Send analysis results to Slack

ANALYSIS=$(curl -s -X POST http://localhost:3001/api/analyze)
SCORE=$(echo $ANALYSIS | jq '.data.quality_score')
ISSUES=$(echo $ANALYSIS | jq '.data.issues | length')

curl -X POST -H 'Content-type: application/json' \
  --data "{\"text\":\"Code Analysis Complete\\nQuality Score: $SCORE\\nIssues Found: $ISSUES\"}" \
  $SLACK_WEBHOOK_URL
```

### Discord Webhook

```python
import requests
import json

def send_discord_notification(analysis_result):
    webhook_url = "YOUR_DISCORD_WEBHOOK_URL"

    embed = {
        "title": "TRAE CLI Analysis Complete",
        "color": 3066993 if analysis_result['data']['quality_score'] > 70 else 15158332,
        "fields": [
            {
                "name": "Quality Score",
                "value": f"{analysis_result['data']['quality_score']:.2f}",
                "inline": True
            },
            {
                "name": "Issues Found",
                "value": str(len(analysis_result['data']['issues'])),
                "inline": True
            },
            {
                "name": "Files Analyzed",
                "value": str(analysis_result['data']['total_files']),
                "inline": True
            }
        ]
    }

    payload = {"embeds": [embed]}
    requests.post(webhook_url, json=payload)

# Usage
analysis = requests.post("http://localhost:3001/api/analyze").json()
send_discord_notification(analysis)
```

## Advanced Integration

### Custom Analysis Rules

```rust
// Extend TRAE CLI with custom analysis rules
use trae_cli::core::analyzer::Analyzer;

struct CustomAnalyzer;

impl Analyzer for CustomAnalyzer {
    fn analyze(&self, content: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Custom rule: Check for TODO comments
        if content.contains("TODO") || content.contains("FIXME") {
            issues.push(Issue {
                file: "custom".to_string(),
                line: 0,
                severity: "info".to_string(),
                message: "TODO comment found".to_string(),
            });
        }

        issues
    }
}
```

### Plugin System

```rust
// TRAE CLI plugin interface
pub trait TraePlugin {
    fn name(&self) -> &str;
    fn analyze(&self, project_path: &str) -> Result<AnalysisResult, Box<dyn std::error::Error>>;
    fn repair(&self, issues: &[Issue]) -> Result<RepairResult, Box<dyn std::error::Error>>;
}

// Load plugins dynamically
fn load_plugins() -> Vec<Box<dyn TraePlugin>> {
    vec![
        Box::new(SecurityAnalyzer::new()),
        Box::new(PerformanceAnalyzer::new()),
        Box::new(CustomAnalyzer::new()),
    ]
}
```

## Troubleshooting Integration

### Connection Issues

```bash
# Test direct connection
curl http://localhost:3001/health

# Test via JARVIXSERVER proxy
curl http://localhost:8080/trae/health

# Check port availability
netstat -tlnp | grep :3001
netstat -tlnp | grep :8080
```

### Timeout Issues

```bash
# Increase timeout for large projects
curl -X POST http://localhost:3001/api/analyze --max-time 300

# Check server logs
Get-Content -Path trae_server.log -Wait
```

### CORS Issues

```javascript
// Add CORS headers for web integration
const corsHeaders = {
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
    'Access-Control-Allow-Headers': 'Content-Type',
};
```

## Performance Optimization

### Load Balancing

```nginx
# nginx.conf for load balancing multiple TRAE CLI instances
upstream trae_backend {
    server localhost:3001;
    server localhost:3002;
    server localhost:3003;
}

server {
    listen 8080;
    location /trae/ {
        proxy_pass http://trae_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### Caching Layer

```bash
# Redis caching for analysis results
redis-cli setex "analysis:$(pwd)" 3600 "$ANALYSIS_RESULT"

# Check cache first
CACHED=$(redis-cli get "analysis:$(pwd)")
if [ -n "$CACHED" ]; then
    echo "Using cached analysis"
    echo $CACHED
else
    ANALYSIS=$(curl -X POST http://localhost:3001/api/analyze)
    redis-cli setex "analysis:$(pwd)" 3600 "$ANALYSIS"
    echo $ANALYSIS
fi
```

---

**TRAE CLI Integration Guide** - Comprehensive integration patterns for CI/CD, monitoring, and enterprise deployments.