# This script is used to create a signed release of Parsec
# using a single command from the pre-built windows artefact.

# This is meant to be run in the `client/electron` directory.

# Stop at first error
$ErrorActionPreference = "Stop"

# Check node version
$expected_node_version = "v18.12.0"
$node_version = node --version
if (-Not ($node_version -eq $expected_node_version)) {
    fnm use $expected_node_version
}

# Cleanup dist directory
if (Test-Path dist) {
    Remove-Item dist -r -force
}

# Install node-modules
npm clean-install

# Build and sign the release
npm run electron:sign

# Create a fresh upload directory
if (Test-Path -Path upload) {
    Remove-Item upload -r -force
}
New-Item "upload" -Type Directory | Out-Null

# Loop over `.exe` and `.exe.blockmap` files
Get-ChildItem "dist" -Filter "Parsec*.exe*" |
Foreach-Object {

    # Copy file to upload directory
    Copy-Item $_.FullName upload\$_

    # Create the .sha256 file without line feed
    (Get-FileHash $_.FullName).Hash | Out-File -Encoding "ASCII" -NoNewline upload\$_.sha256

    # Add a `\n` to the end of the .sha256 file
    Add-Content -Value "`n" -NoNewline upload\$_.sha256
}

# Output the items to upload
Get-ChildItem upload
