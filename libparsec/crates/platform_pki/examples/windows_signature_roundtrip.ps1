cargo build --examples -p libparsec_platform_pki

$certificate_fingerprint = cargo run -p libparsec_platform_pki --example get_certificate_der | Select-String -Pattern '^fingerprint: .*$' | ForEach-Object { $_.line.Split(': ')[-1] }

$sign_algo, $signature = cargo run -p libparsec_platform_pki --example sign_message -- $certificate_fingerprint --content-file Cargo.toml | Select-String -Pattern 'with algorithm .*','^Signature: .*$' | ForEach-Object { $_.line.Split(': ')[-1] }

echo "Signing content of Cargo.toml"
echo "using algo: $sign_algo"
echo "and certificate: $certificate_fingerprint"
echo "Result in signature: $signature"

cargo run -p libparsec_platform_pki --example verify_message -- --certificate-hash $certificate_fingerprint --content-file Cargo.toml $sign_algo $signature
