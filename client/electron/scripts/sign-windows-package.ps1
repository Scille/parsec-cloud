<#
.SYNOPSIS
  Create a signed release of Parsec

.DESCRIPTION
  This script is used to create a signed release of Parsec
  using a single command from the pre-built windows artefact.

  This is meant to be run in the `client/electron` directory.

.PARAMETER Hardened
    Perform the signature for an hardened build
#>
[CmdletBinding()]
param(
    [switch] $Hardened
)
# Check node version
$expected_node_version = "v22.14.0"
$node_version = node --version
$need_to_install_node = ($LastExitCode -ne 0) -or (-Not ($node_version -eq $expected_node_version))

# Stop at first error
# Note this only works for shell commands, executables return code must still
# be checked manually :/
# NB: We set it after node checks because else it will stop at the first failed check preventing installation from fnm
$ErrorActionPreference = "Stop"

if ($need_to_install_node) {
    fnm env --use-on-cd --shell powershell | Out-String | Invoke-Expression
    fnm use $expected_node_version
} else {
    echo "Node is installed with the correct version"
}

if ($Hardened) {
    echo "Doing an hardened build!"
}

# Cleanup dist directory
if (Test-Path dist) {
    Remove-Item dist -r -force
}

Set-PSDebug -Trace 1
# Install node-modules
npm clean-install

# Build and sign the release
if ($Hardened) {
    node package.js --mode prod --sign --hardened
} else {
    npm run electron:sign
}
Set-PSDebug -Trace 0
if ($LastExitCode -ne 0) {
    exit $LastExitCode
}

# Create a fresh upload directory
if (Test-Path -Path upload) {
    Remove-Item upload -r -force
}
New-Item "upload" -Type Directory | Out-Null

# Rename `latest.yml` file to include architecture
if ($Hardened) {
    Copy-Item dist\latest.yml dist\latest-hardened-x86_64.yml
} else {
    Copy-Item dist\latest.yml dist\latest-win-x86_64.yml
}


# Loop over 3 files:
# - Parsec_<version>_win_x86_64.exe
# - Parsec_<version>_win_x86_64.exe.blockmap
# - latest-win-x86_64.yml
Get-ChildItem "dist\*" -Include "Parsec*.exe*","latest-*.yml" |
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
