# TPEK bootstrap organization evolutions

From [ISSUE-2373](https://github.com/Scille/parsec-cloud/issues/2373)

## 1 - Changes

backend datamodel should be modified to include:

- tpek verify key x509 certificate in the `organization` table (as an optional blob column)
- new table tpek:

```sql
CREATE TYPE tpek_service AS ENUM ('SEQUESTRE', 'WEBHOOK');

CREATE TABLE tpek(
    _id SERIAL PRIMARY KEY,
    service_type tpek_service NOT NULL,
    service_id TEXT NOT NULL,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    encryption_key BYTEA,  -- Encryption key signed by the certificate
    webhook_url TEXT, -- NULL if service_type != WEBHOOK

    UNIQUE(organization, service_id)
);
```

Notes:

- encryption_key is a x509 certificate on a asymmetrical encryption key
- updating the tpek for a service overwrite the previous encryption_key
- backend should check the encryption key validity before storing it in the database

## 2 - New cli command

```shell
$ parsec backend tpek register
...
$ parsec backend tpek list
<display service name, type, webhook url and maybe encryption key hash ?>
```
