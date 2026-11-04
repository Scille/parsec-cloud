<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# How to Update `SNAPCRAFT_CREDENTIALS`

The `SNAPCRAFT_CREDENTIALS` value is used in the CI workflow to release new versions
of the Parsec snap (see [`.github/workflows/publish.yml`](/.github/workflows/publish.yml)).

This credential is configured in the GitHub repository as an
[**Action secret**](https://github.com/Scille/parsec-cloud/settings/secrets/actions).

## Updating the Credentials

To update this value:

1. Run the following command to export your Snapcraft credentials:

   ```bash
   snapcraft export-login --snaps parsec --acls package_upload -
   ```

2. Copy the exported login credentials from the output.

3. Use it to update the `SNAPCRAFT_CREDENTIALS` secret in the
   [GitHub repository settings](https://github.com/Scille/parsec-cloud/settings/secrets/actions).
