<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->


# Generate blob for tests


If you need to generate a blob for your tests, you can use the following process:

- Generate what you need (expected data)
- Dump (serialize) the data and format the serialized data in hexadecimal (don't commit the modifications you have just done)
- Use `hex!` macro and store the value in a variable, each row should contain `70` characters (see [rfc 1009](../rfcs/1009-hexstring-format.md))
- Load the variable
- Test that the loaded value is the same as the expected one
- Test roundtrip (dump & load), the loaded value should be the same as the previous one

> Note: We can't assert on the dumped value because the order is not guaranteed with `msgpack`
