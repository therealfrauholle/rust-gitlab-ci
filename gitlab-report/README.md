# gitlab-report

A command line utility to generate GitLab compatible reports from cargo JSON output.

Supported formats:

| Input  | Output
|:-------|:---
| test   | JUnit
| test   | OpenMetrics
| clippy | Code Climate
| clippy | OpenMetrics
| bench  | OpenMetrics
| audit  | GitLab SAST

## Usage

```shell
cargo test --no-fail-fast -- -Z unstable-options --format json | gitlab-report -p test > report.xml
cargo clippy --message-format=json | gitlab-report -p clippy > gl-code-quality-report.json
cargo bench -- -Z unstable-options --format json | gitlab-report -p bench > metrics.txt
cargo audit | gitlab-report -p audit > gl-sast-report.json
```