<#
.SYNOPSIS
  Create a test PKI

.DESCRIPTION
  Create 3 certificates in the windows certificate store:
  - a CA self-signed certificate, in the ROOT store
  - a certificate for Alice signed by the CA, in the MY store
  - a certificate for Bob signed by the CA, in the MY store
.PARAMETER OnlyCleanup
    Only perform the cleanup operation without creating a new test PKI
#>
[CmdletBinding()]
param(
    [switch] $OnlyCleanup
)

function Cleanup-Test-PKI {
    Get-ChildItem Cert:\CurrentUser\Root | Where-Object Subject -Match "Parsec Test CA"    | Remove-Item -Verbose
    Get-ChildItem Cert:\CurrentUser\CA   | Where-Object Subject -Match "Parsec Test CA"    | Remove-Item -Verbose
    Get-ChildItem Cert:\CurrentUser\My   | Where-Object Subject -Match "Parsec Test CA"    | Remove-Item -Verbose
    Get-ChildItem Cert:\CurrentUser\My   | Where-Object Subject -Match "alice Parsec Test" | Remove-Item -Verbose
    Get-ChildItem Cert:\CurrentUser\My   | Where-Object Subject -Match "bob Parsec Test"   | Remove-Item -Verbose
    echo "Cleanup done"
}

# Perform cleanup
echo "Removing Test PKI if any"
Cleanup-Test-PKI

if ($OnlyCleanup) {
    exit
}

# Cleanup test pki on Ctrl+C, Stop Event or Shutdown
$null = Register-EngineEvent -SourceIdentifier PowerShell.Exiting -SupportEvent -Action { Cleanup-Test-PKI }

echo "Creating root certificate 'cert:\CurrentUser\CA\test_ca'"
# Create self-signed CA certificate
$CAParam = @{
    Subject = 'CN=Parsec Test CA'
    FriendlyName = 'CA Parsec Test'
    KeyFriendlyName = 'CA Parsec Test key'
    KeyUsageProperty = @(
        [Microsoft.CertificateServices.Commands.KeyUsageProperty]::Sign
    )
    KeyUsage = @(
        [Microsoft.CertificateServices.Commands.KeyUsage]::CertSign,
        [Microsoft.CertificateServices.Commands.KeyUsage]::CRLSign,
        [Microsoft.CertificateServices.Commands.KeyUsage]::DigitalSignature
    )
    KeyAlgorithm = 'RSA'
    KeyLength = 2048
    # We set the param `NotBefore` as the default value make the CA valid 10 min after its creation.
    NotBefore = (Get-Date)
    NotAfter = (Get-Date).AddHours(24)
    # We cannot add it directly to `CA` store as the command only allow for `My` store
    CertStoreLocation = 'Cert:\CurrentUser\My'
    # Use RSA-PSS-SHA256 over RSASSA-SHA256 for signature
    AlternateSignatureAlgorithm = $true
}
$test_ca = New-SelfSignedCertificate @CAParam

echo "Remove the copy of the Test Root cert. that was added in the intermediate cert store"
Get-ChildItem Cert:\CurrentUser\CA | Where-Object Thumbprint -Match $test_ca.Thumbprint | Remove-Item -Verbose

echo "Export the certificate to a temporary file"
$tempcert = Export-Certificate -Cert $test_ca -FilePath test_ca.crt

# This is required to have our newly generated certificate marked as trusted.
echo "Add Test CA to 'Root' cert. store"
certutil.exe -v -user -addstore Root $tempcert

# Remove temporary file
Remove-Item $tempcert

$SharedUserCertificate = @{
    Signer = $test_ca
    KeyAlgorithm = 'RSA'
    KeyLength = 2048
    NotBefore = $test_ca.NotBefore
    NotAfter = $test_ca.NotAfter
    CertStoreLocation = 'Cert:\CurrentUser\My'
    # Use RSA-PSS-SHA256 over RSASSA-SHA256 for signature
    AlternateSignatureAlgorithm = $true
}

echo "Create certificate 'cert:\CurrentUser\My\test_ca_alice'"
$test_alice = New-SelfSignedCertificate @SharedUserCertificate -Subject "CN=alice Parsec Test,E=alice@example.com" -FriendlyName 'alice Parsec Test'
echo "Create certificate 'cert:\CurrentUser\My\test_ca_bob'"
$test_bob = New-SelfSignedCertificate @SharedUserCertificate -Subject "CN=bob Parsec Test,E=bob@example.com" -FriendlyName 'bob Parsec Test'

try {
    Write-Output "Press any key to continue..."
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
}
finally {
    Cleanup-Test-PKI
}
