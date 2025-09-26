cargo build --examples -p libparsec_platform_pki

$certificate_fingerprint = cargo run -p libparsec_platform_pki --example get_certificate_der | Select-String -Pattern '^fingerprint: .*$' | ForEach-Object { $_.line.Split(': ')[-1] }

$content_string = "Hello world"

if ($content_string.Length > 2048) {
    echo "The content to encrypt is too big for the 2048-RSA key"
    exit
}

$encrypt_algo, $ciphered = cargo run -p libparsec_platform_pki --example encrypt_message -- $certificate_fingerprint --content $content_string | Select-String -Pattern 'using the algorithm .*','^Encrypted data: .*$' | ForEach-Object { $_.line.Split(': ')[-1] }

echo "Encrypt message: $content_string"
echo "using algo: $encrypt_algo"
echo "and certificate: $certificate_fingerprint"
echo "Result in ciphered: $ciphered"

cargo run -p libparsec_platform_pki --example decrypt_message -- $certificate_fingerprint --content $ciphered --algorithm $encrypt_algo
