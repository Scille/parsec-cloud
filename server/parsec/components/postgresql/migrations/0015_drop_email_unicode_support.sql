-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Unicode support for email is messy and opens the door to homoglyph attacks
-- while covering few legitime usecase (basically everybody use ASCII-only email
-- addresses, and legitimate unicode email address can also be encoded in ASCII
-- using Punycode).
--
-- For this reason we dropped support for unicode, however this means we have to
-- make sure the database doesn't contain email address containing unicode.
-- In such case, the whole organization is going to become invalid since the
-- now-invalid email is used in a user certificate.
--
-- There is no way to automatically correct the organization (since the user certificate
-- is signed), so instead the organization has to be recreated from scratch :/
-- Hopefully we expect basically no organization is to be impacted considering how
-- exotic unicode email addresses are...
-------------------------------------------------------

DO $$
DECLARE
    non_ascii_emails TEXT;
BEGIN
    -- Check `human` table

    SELECT string_agg(email, E'\n') INTO non_ascii_emails
    FROM human
    WHERE email !~ '^[\x00-\x7F]*$';  -- ASCII range

    ASSERT non_ascii_emails IS NULL,
        format(
            E'Email with non-ASCII characters is no longer supported, you should manually remove them and recreate the organization they belong to...\nNon-ASCII Email(s) found in table `human`:\n%s',
            non_ascii_emails
        );

    -- Check `invitation` table

    SELECT string_agg(user_invitation_claimer_email, E'\n') INTO non_ascii_emails
    FROM invitation
    WHERE
        user_invitation_claimer_email IS NOT NULL
        AND user_invitation_claimer_email !~ '^[\x00-\x7F]*$';  -- ASCII range

    ASSERT non_ascii_emails IS NULL,
        format(
            E'Email with non-ASCII characters is no longer supported, you should manually remove them and recreate the organization they belong to...\nNon-ASCII Email(s) found in table `invitation`:\n%s',
            non_ascii_emails
        );

    -- Check `account` table, this should never be an issue since this table is not yet in production!

    SELECT string_agg(email, E'\n') INTO non_ascii_emails
    FROM account
    WHERE email !~ '^[\x00-\x7F]*$';  -- ASCII range

    ASSERT non_ascii_emails IS NULL,
        format(
            E'Email with non-ASCII characters is no longer supported, you should manually remove them and recreate the organization they belong to...\nNon-ASCII Email(s) found in table `account`:\n%s',
            non_ascii_emails
        );

    -- Check `account_create_validation_code` table, this should never be an issue since this table is not yet in production!
    SELECT string_agg(email, E'\n') INTO non_ascii_emails
    FROM account_create_validation_code
    WHERE email !~ '^[\x00-\x7F]*$';  -- ASCII range

    ASSERT non_ascii_emails IS NULL,
        format(
            E'Email with non-ASCII characters is no longer supported, you should manually remove them and recreate the organization they belong to...\nNon-ASCII Email(s) found in table `account_create_validation_code`:\n%s',
            non_ascii_emails
        );

END $$;
