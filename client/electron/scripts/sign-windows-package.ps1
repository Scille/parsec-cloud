# This script is used to create a signed release of Parsec
# using a single command from the pre-built windows artefact.

# This is meant to be run in the `client/electron` directory.

# Stop at first error
# Note this only works for shell commands, executables return code must still
# be checked manually :/
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
if ($LastExitCode -ne 0) {
    exit $LastExitCode
}

# Create a fresh upload directory
if (Test-Path -Path upload) {
    Remove-Item upload -r -force
}
New-Item "upload" -Type Directory | Out-Null

# Rename `latest.yml` file to include architecture
Copy-Item dist\latest.yml dist\latest-win-x86_64.yml

# Loop over 3 files:
# - Parsec_<version>_win_x86_64.exe
# - Parsec_<version>_win_x86_64.exe.blockmap
# - latest-win-x86_64.yml
Get-ChildItem "dist\*" -Include "Parsec*.exe*","latest-win*.yml" |
Foreach-Object {
    $Source = $_.FullName
    $Destination = Join-Path ".\upload" $_.Name
    $DestinationSha256 = $Destination + ".sha256"

    # Copy file to upload directory
    Copy-Item $Source $Destination

    # Create the .sha256 file without line feed
    (Get-FileHash $Source).Hash | Out-File -Encoding "ASCII" -NoNewline $DestinationSha256

    # Add a `\n` to the end of the .sha256 file
    Add-Content -Value "`n" -NoNewline $DestinationSha256
}

# Output the items to upload
Get-ChildItem upload
