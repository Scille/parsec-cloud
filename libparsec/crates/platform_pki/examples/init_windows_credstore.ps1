# Create 3 certificates in the windows certificate store:
# - a CA self-signed certificate, in the ROOT store
# - a certificate for Alice signed by the CA, in the MY store
# - a certificate for Bob signed by the CA, in the MY store
# A cleanup can also be performed by provided the cleanup option.

# Use "cleanup" to remove all the certificates starting with "test_"
param ($action)

# Perform cleanup
if ($action -eq "cleanup") {
    echo "Removing Test PKI"
    Get-ChildItem Cert:\CurrentUser\CA   | Where-Object Subject -Match "Parsec Test CA"    | Remove-Item
    Get-ChildItem Cert:\CurrentUser\My   | Where-Object Subject -Match "Parsec Test CA"    | Remove-Item
    Get-ChildItem Cert:\CurrentUser\My   | Where-Object Subject -Match "Alice Parsec Test" | Remove-Item
    Get-ChildItem Cert:\CurrentUser\My   | Where-Object Subject -Match "Bob Parsec Test"   | Remove-Item
    Get-ChildItem Cert:\CurrentUser\Root | Where-Object Subject -Match "Parsec Test CA"    | Remove-Item
    exit
}

# Wrong parameter
if ($null -ne $action) {
    Write-Output "Wrong parameter, use 'cleanup' or nothing."
    exit
}

echo "Creating Test CA"
# Create self-signed CA certificate
# We set the param `NotBefore` as the default value make the CA valid 10 min after its creation.
# We cannot add it directly to `CA` store as the command only allow for `My` store
$test_ca = New-SelfSignedCertificate -Subject "CN=Parsec Test CA" -KeyUsage CertSign,DigitalSignature -CertStoreLocation cert:\CurrentUser\My -NotBefore ([DateTime]::UtcNow)

echo "Remove the intermediate certificate"
Get-ChildItem Cert:\CurrentUser\CA | Where-Object Thumbprint -Match $test_ca.Thumbprint | Remove-Item

echo "Export the certificate to a temporary file"
$tempcert = Export-Certificate -Cert $test_ca -FilePath test_ca.crt

echo "Add Test CA to `CA` cert. store"
certutil.exe -addstore Root $tempcert

# Remove temporary file
Remove-Item $tempcert

echo "Create certificate for alice"
$test_alice = New-SelfSignedCertificate -Signer $test_ca -Subject "CN=alice Parsec Test,E=alice@example.com" -CertStoreLocation cert:\CurrentUser\My
echo "And a certificate for Bob"
$test_bob = New-SelfSignedCertificate -Signer $test_ca -Subject "CN=bob Parsec Test,E=bob@example.com" -CertStoreLocation cert:\CurrentUser\My

echo ""
echo "IMPORTANT: we have added a test certificate in your Root trust store."
echo "You should remove it ASAP once you're done testing."
echo "You can call the script again with the argument 'cleanup' to remove it."
