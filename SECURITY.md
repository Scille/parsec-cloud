# Security Policy

Thanks for helping make Parsec safe for everyone.

We take the security of our software products and services seriously, including
all of the open source code repositories managed through our GitHub
organization, [Scille](https://github.com/Scille/).

If you believe you have found a security vulnerability in any Scille-owned
repository, please report it to us through **coordinated disclosure** as
described below.

## Reporting a vulnerability

Security vulnerabilities can be reported through the following channels:

- [GitHub's private vulnerability reporting tool](https://github.com/Scille/parsec-cloud/security/advisories/new)
- By sending an email to [security@parsec.cloud](mailto:security@parsec.cloud)

> [!CAUTION]
> **Please DO NOT report security vulnerabilities through public GitHub issues,
> discussions, or pull requests.**

Include as much of the information listed below as you can to help us better
understand and resolve the issue:

- A clear description of the issue
- The steps you took to reproduce the issue
- The configuration required to reproduce the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- The source file(s) related to the manifestation of the issue
- The impact of the issue, including how an attacker might exploit the issue
- The mitigations for the issue (if possible)
- Proof-of-concept or exploit code (if possible)

This information will help us triage your report more quickly.

See GitHub's [best practices when reporting a vulnerability](https://github.blog/security/vulnerability-research/coordinated-vulnerability-disclosure-cvd-open-source-projects/#best-practices-when-reporting-a-vulnerability).

## Coordinated vulnerability disclosure (CVD)

Regardless of the method chosen to report the vulnerability, coordinated disclosure will be carried out as described below.

> [!NOTE]
> *Coordinated disclosure is a vulnerability disclosure model in which a vulnerability or an issue
> is disclosed to the public only after the responsible parties have been allowed sufficient time
> to patch or remedy the vulnerability or issue.*
>
> [Coordinated vulnerability disclosure (Wikipedia)](https://en.wikipedia.org/wiki/Coordinated_vulnerability_disclosure)

When the vulnerability management team (VMT) receives a security bug report, they will assign it to a
primary handler (the repository administrator, the organization administrator, or the organization's
security managers), who may discuss the report with you in order to better understand the vulnerability,
and determine the affected versions.

The process can be summarized as follows:

1. **Acknowledge** - The VMT will acknowledge receipt of your report within a few days.
2. **Assessment** - The VMT assesses your report and decides if the issue is working-as-intended, a bug,
  a feature request, or a security issue. The VMT will respond with their assessment.
   - If they determine it is not a security threat, they may stop the disclosure process and ask you to
  submit it as a public non-security issue instead.
   - If the security issue is confirmed, the VMT will update or open a draft Security Advisory on GitHub
    and will add you as collaborator (unless you tell us otherwise).
3. **Patch creation** - If the vulnerability is confirmed, the project maintainers will begin developing a
   patch in private. At this step, they may reach out to you to help develop and/or test the patch.
4. **CVE assignment** - The VMT may request a CVE entry if the vulnerability is eligible for one.
5. **Public disclosure** - Once a patch is published, maintainers will notify users and direct them to update.

These fixes will be released as soon as possible depending on complexity.

### Access and visibility

Until it is published, the draft security advisory will only be visible to the maintainers and invited collaborators.

Once published, advisories can be found in the [Security Advisories](https://github.com/Scille/parsec-cloud/security/advisories/).

### Credit

You will be automatically credited as a contributor once the advisory is published.
