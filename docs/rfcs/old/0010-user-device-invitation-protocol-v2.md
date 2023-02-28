# User/Device invitation protocol v2

From [ISSUE-1037](https://github.com/Scille/parsec-cloud/issues/1037)

In line with [RFC-0008](0008-smoothening-the-invite-process.md) & [RFC-0009](0009-use-email-as-main-human-identifier.md), here is a draft of the new invitation protocol:

```txt
Invitee (Alice)              Inviter (Bob)

--- Diffie-Hellman key exchange ---

Generate key Ask/Apk
Apk -->

                             Generate key Bsk/Bpk
                             Compute s = f(Bsk, Apk)
                             <-- Bpk

Compute s = f(Ask, Bpk)

--- Shared key verification ---

Generate 32bits nonce An
hAn = hmac(s, An)
hAn -->
                            TODO: useful to send hBn == hmac(s, Bn) ???
                            Generate 32bits nonce Bn
                            <-- Bn

An -->
                            Verifies hAn == hmac(s, An)
                            Computes Bsas = hmac(s, An | Bn)[:20b]
                              (first 20bits of the hmac)
                            Transmit Bsas as 7 digit number by out of band canal

Receive Bsas
Verifies Bsas == hmac(s, An | Bn)[:20b]
*** Invitee knows canal is secure ***
*Invitee ready* -->

Compute Asas = hmac(s, An | Bn)[20b:40b]
  (bits ]20, 40] of the hmac)
Transmit Asas as 7 digit
  number by out of band canal

                            Receive Asas
                            Verifies Asas == hmac(s, An | Bn)[20b:40b]
                            *** Inviter knows canal is secure ***
                            <-- *Inviter ready*

--- Actual client or device invitation ---

User/Device pub key -->
 (Device pub key in case of Device invitation)

                           Create User certificate & send it to backend
                           <-- root public key
                              (+ user private key in case of Device invitation)
```

**Few key points:**

It is heavily based on <https://tools.ietf.org/id/draft-ietf-dnssd-pairing-03.html>

There is only one nonce-hash passed from invitee to inviter (`hAn`). This is to follow what is done in the IETF draft. However I think we should investigate better the lack of need for a inviter to invitee nonce-hash.

Unlike the current protocol, there is 2 token to pass (inviter send to invitee, then invitee send to inviter)

Though the token could be replace by a visual comparison between invitee and inviter, it's much safer to force them to provide each other a different code so they cannot "just click ok and go on"

The 2 tokens are each 20 bits long, so we end up with 5bytes of data. This is pretty big (~10¹² possibilities) but oblige to pass two 7 digits numbers.
We could use 4 digits numbers instead (12bits tokens) and still have 16 millions possibilities (still much higher than 1 million possibilities of the single 7 digit token used in the IETF draft).

In theory 4 digits numbers provides 13bits (8096 possibilities), however the blake2b hash that we will most likely use can output hash with a size between 1 and 64 bytes.
Using two 12bits tokens allow us to use a hash size of 3 bytes, were 13bits tokens would consume only 26bits out of the 32bits of a 4bytes hash... In theory this wouldn't be an issue, but I would prefer hard evidence before mangling cryptographic output.
