# Security Policy

## Supported versions

Security fixes are applied to the latest published version only.

| Version | Supported |
|---------|-----------|
| latest  | Yes       |
| older   | No        |

## Reporting a vulnerability

Please **do not** open a public GitHub issue for security vulnerabilities.

Instead, report them privately via [GitHub Security Advisories](https://github.com/anthonyoteri/cargo-rigtest/security/advisories/new).

Include:

- A description of the vulnerability and its potential impact.
- Steps to reproduce or a proof-of-concept.
- The version(s) affected.

You can expect an initial response within **7 days**. If the report is accepted, a fix will be prepared and a CVE requested where appropriate. You will be credited in the advisory unless you prefer to remain anonymous.

## Scope

cargo-rigtest is a developer tool that runs in a trusted local or CI environment. It executes test binaries that you compile from your own source code. The primary security surface is:

- Subprocess execution of compiled test binaries
- Environment variable handling for global state handoff between processes
- Parsing of `cargo test --message-format=json` output

Reports outside this scope (e.g. vulnerabilities in transitive dependencies) are still welcome but may be redirected upstream.
