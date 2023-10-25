// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use hex_literal::hex;
use rstest::rstest;

use libparsec_crypto::{
    CryptoError, SequesterKeySize, SequesterPrivateKeyDer, SequesterPublicKeyDer,
    SequesterSigningKeyDer, SequesterVerifyKeyDer,
};

/// Generated with `openssl genrsa -out pkey-1024.pem 1024`
const PRIVATE_KEY_PEM_1024: &str = r#"-----BEGIN PRIVATE KEY-----
MIICdgIBADANBgkqhkiG9w0BAQEFAASCAmAwggJcAgEAAoGBALLcAKPDtcaJsGnz
9AxJTSpb4xOxA0+/Hf4O7uDzbPvPYkQAJWzGYNUIR4JzijBF11tYTBlDvATHEj1o
rAzvJTtO6Neb0J2hkWLcwINmImm3tiyzhYL4owIZBHsIfBG2AYSwST4MHIsdEPnX
5qLrWv9m9+4YMDGV87zHKrVyB+v9AgMBAAECgYBBfQRO8g3QkAGkCcrF5OD4LYTL
ZPjNbjDRIS6d9wNkf95+/360gT5bQhjMzvk+C5R6wa27Ym2pYipvia/VXIrIuwoP
GDKyUg/B2SrDIBgLlleosVmOZwERnY939ShaxXA/TAqT4eXr5uwXm8z/YkledzTV
iZ2G0ty962SJI9KbEQJBAOsrXVrVvCjK4WVFBcFqKw2Cz6I3+PEPcOXnozMwKKre
XeT54rPoqfkkoFOPcRngiPPBHxJYMZxZ2gPWN6MkxmMCQQDCs8SRRd/XkwrH9Wga
+kO3sY9pcWNGnHq5psqOEhaOqxozzE5ztT9FCcSAUqMf8onnNXpU+I4o7lQwQACZ
dmIfAkAmtgiz/yLuBBd+OBJueC+GFdZf+Z6877HB5pNyxaasGdaS7p9mxhHUtTa/
ComvnMpudYfL2UCxYAkHQKf/7vnJAkEAk1Zqd+zCmWXikLK7Fz8vo4CwoAB4OeUM
UhVPzvcNLuV4LJ58976+pEXh96GRZAmsJdUoP8jf+0VvXBvy2C7nzQJAGLjFefMr
vXTMHxDHhuHg4ZJSbJ5BNMZb/HZ5m4KQCt9Gelx/sxZOt3VlCrquCFALxmkeYMhX
W1pav08hVpEcTg==
-----END PRIVATE KEY-----
"#;

/// Generated with `openssl rsa -in pkey-1024.pem -outform der -out pkey-1024.der`.
/// Formatted with `xxd -c 32 -g 32 pkey-1024.der | cut -f 2 -d ' '`.
const PRIVATE_KEY_DER_1024: &[u8] = &hex!(
    "30820276020100300d06092a864886f70d0101010500048202603082025c0201"
    "0002818100b2dc00a3c3b5c689b069f3f40c494d2a5be313b1034fbf1dfe0eee"
    "e0f36cfbcf624400256cc660d5084782738a3045d75b584c1943bc04c7123d68"
    "ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb38582f8a30219047b"
    "087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72a"
    "b57207ebfd0203010001028180417d044ef20dd09001a409cac5e4e0f82d84cb"
    "64f8cd6e30d1212e9df703647fde7eff7eb4813e5b4218cccef93e0b947ac1ad"
    "bb626da9622a6f89afd55c8ac8bb0a0f1832b2520fc1d92ac320180b9657a8b1"
    "598e6701119d8f77f5285ac5703f4c0a93e1e5ebe6ec179bccff62495e7734d5"
    "899d86d2dcbdeb648923d29b11024100eb2b5d5ad5bc28cae1654505c16a2b0d"
    "82cfa237f8f10f70e5e7a3333028aade5de4f9e2b3e8a9f924a0538f7119e088"
    "f3c11f1258319c59da03d637a324c663024100c2b3c49145dfd7930ac7f5681a"
    "fa43b7b18f697163469c7ab9a6ca8e12168eab1a33cc4e73b53f4509c48052a3"
    "1ff289e7357a54f88e28ee543040009976621f024026b608b3ff22ee04177e38"
    "126e782f8615d65ff99ebcefb1c1e69372c5a6ac19d692ee9f66c611d4b536bf"
    "0a89af9cca6e7587cbd940b160090740a7ffeef9c902410093566a77ecc29965"
    "e290b2bb173f2fa380b0a0007839e50c52154fcef70d2ee5782c9e7cf7bebea4"
    "45e1f7a1916409ac25d5283fc8dffb456f5c1bf2d82ee7cd024018b8c579f32b"
    "bd74cc1f10c786e1e0e192526c9e4134c65bfc76799b82900adf467a5c7fb316"
    "4eb775650abaae08500bc6691e60c8575b5a5abf4f2156911c4e"
);

/// Generated with `openssl rsa -in pkey-1024.pem -pubout -out pubkey-1024.pem`
const PUBLIC_KEY_PEM_1024: &str = r#"-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCy3ACjw7XGibBp8/QMSU0qW+MT
sQNPvx3+Du7g82z7z2JEACVsxmDVCEeCc4owRddbWEwZQ7wExxI9aKwM7yU7TujX
m9CdoZFi3MCDZiJpt7Yss4WC+KMCGQR7CHwRtgGEsEk+DByLHRD51+ai61r/Zvfu
GDAxlfO8xyq1cgfr/QIDAQAB
-----END PUBLIC KEY-----
"#;

/// Generated with `openssl rsa -in pkey-1024.pem -pubout -outform der -out pubkey-1024.der`
const PUBLIC_KEY_DER_1024: &[u8] = &hex!(
    "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689"
    "b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a"
    "3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62c"
    "b38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195"
    "f3bcc72ab57207ebfd0203010001"
);

const PRIVATE_KEY_PEM_2048: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCnDVsZGa7iu45E
2d++JupEgmdlo8EGFjfXJzVgXduWT42rmxGEKCARzHt5RK0EjSsfmLg4uQqvVlEc
lUtN4HR466nU6tqnf+PLjYJNVQQVh1rRM27N97gfRxuOVwkb6tAMIFUytWRyQd9C
FEzMpLJebFeZc/dWQ76gOXhBY71L5/7O+m8xugwIpex8qvao+aVgHaQqGNHJN+CB
Zys2/GzMO/6ypbbhy6nr/OSknvc00I7XiQDynq77fciZNuqPIa7Za+N14rIrUsyK
bCEc8RH0sn2wcdieVXHPdvF3ZasFbD5tNJOHo//SplOLEwhdYAGs7NEcjEOmXksk
8t0gCJcxAgMBAAECggEAfJIqF6qeTnd8XP13qo3MbnAr+JBHdWmGFIwpcoYrJIiI
ZaqKQlnFqGAqx0CeIOHAgZxZ6/qY7C1a6QyvjhBN6ooaKCtGCFgHH5iTzWUTWZaI
863999u9Up2s/9UJducAY7qMrfa0Q7u5Z8ZJiuVaGa2DGWxvycVU1phkg6aD16Cg
nZ3+9oWbaBfELPEFdktYwa5fwflOMN6+Fw7isIVlOC4tvGpiR1scXKEwjr2P5a0P
D0s6ca4D9xlCrlJ6CpdyKbLwz5ya+gBwjW3OJYI8+NzIw5SwmnnZoP3MqI96ME70
RGBfyqPs4DZjEaqWSSiaDMu10nndEvdakFwgCBzCAQKBgQDeIAouUgnG53RCjukN
mLTHLDpuQk7yycmVcjqWgq2Dkg2FxGbl7f9p6vLDicbMieXYbSTbt5rYTnU6c5j0
o2uAiGsUK8E7JsD8qid7XV7M6r0mTCAqP16SEcf9oSureYFXIWq6pkgnQLS6LVw6
rWwG+uaaCw3qjQvlPk17pvKGYQKBgQDAhzf5EJG3LICmgSKsz6qS/G82N8gxqCwC
Gf8egXGweQSSYrUaemtsAHHbVNIxEqPoedpj+ErofvYo2exNbQVyp33rC1kGNqqT
YTzsV83jp0vv9cn6LwgeK+5mfuNTs13H/YfKpdfJRgoo78Ot7c//FpcM4iwtcFEm
CDhD7DAi0QKBgCSBmYrBWvAAzD/AIxdj+Jofb779UOJgPaw9KNnhwki0cVqBy/OQ
KQEsZxeyBiVJqKfoUD14MI/KXUPtCb9zOFDYmtim1Ew3c1JTWMfSsaj2D35C1qp6
3b0eBQNvQLEe+B3s1RC2EXe6D7nliJnwNKf1Bn9cv73DzqevlKZ06rkhAoGBAJAF
VuCw/WAaIU7s8QR3AHGd9o+HYPGMjQcxbT/jsylBDiN3RWS+SRdsqb9aDzZ4bDae
2LjJ5zdOqlPa2UL+GAIxuycVwQ293RIMvmdKM5Fl949UvTey8Q6o7Fit+N4HIlNV
m7wDHY3KIBWJbkbwzBRVUL9pfYSsLXgW1AgzabrRAoGAUW82mYl9HAwQOBPb40OK
j2qKisC4VA8vR/Uco9yda5XT7iqG3htvUa+Ugj5r4XFRJvOyQG2WUiOWwGYlzfz2
1Ogu8O8RHcaJafAIMTJJtjsO8pIf6Y7aWHA3Ze3to+01fTkGB2I8tDKc3fn+iKlX
z0mxbH0U7CoeRzfgJ6UmIio=
-----END PRIVATE KEY-----
"#;

const PRIVATE_KEY_DER_2048: &[u8] = &hex!(
    "308204bd020100300d06092a864886f70d0101010500048204a7308204a302010002820101"
    "00a70d5b1919aee2bb8e44d9dfbe26ea44826765a3c1061637d72735605ddb964f8dab9b11"
    "84282011cc7b7944ad048d2b1f98b838b90aaf56511c954b4de07478eba9d4eadaa77fe3cb"
    "8d824d550415875ad1336ecdf7b81f471b8e57091bead00c205532b5647241df42144ccca4"
    "b25e6c579973f75643bea039784163bd4be7fecefa6f31ba0c08a5ec7caaf6a8f9a5601da4"
    "2a18d1c937e081672b36fc6ccc3bfeb2a5b6e1cba9ebfce4a49ef734d08ed78900f29eaefb"
    "7dc89936ea8f21aed96be375e2b22b52cc8a6c211cf111f4b27db071d89e5571cf76f17765"
    "ab056c3e6d349387a3ffd2a6538b13085d6001acecd11c8c43a65e4b24f2dd200897310203"
    "010001028201007c922a17aa9e4e777c5cfd77aa8dcc6e702bf89047756986148c2972862b"
    "24888865aa8a4259c5a8602ac7409e20e1c0819c59ebfa98ec2d5ae90caf8e104dea8a1a28"
    "2b460858071f9893cd6513599688f3adfdf7dbbd529dacffd50976e70063ba8cadf6b443bb"
    "b967c6498ae55a19ad83196c6fc9c554d6986483a683d7a0a09d9dfef6859b6817c42cf105"
    "764b58c1ae5fc1f94e30debe170ee2b08565382e2dbc6a62475b1c5ca1308ebd8fe5ad0f0f"
    "4b3a71ae03f71942ae527a0a977229b2f0cf9c9afa00708d6dce25823cf8dcc8c394b09a79"
    "d9a0fdcca88f7a304ef444605fcaa3ece0366311aa9649289a0ccbb5d279dd12f75a905c20"
    "081cc20102818100de200a2e5209c6e774428ee90d98b4c72c3a6e424ef2c9c995723a9682"
    "ad83920d85c466e5edff69eaf2c389c6cc89e5d86d24dbb79ad84e753a7398f4a36b80886b"
    "142bc13b26c0fcaa277b5d5ecceabd264c202a3f5e9211c7fda12bab798157216abaa64827"
    "40b4ba2d5c3aad6c06fae69a0b0dea8d0be53e4d7ba6f2866102818100c08737f91091b72c"
    "80a68122accfaa92fc6f3637c831a82c0219ff1e8171b079049262b51a7a6b6c0071db54d2"
    "3112a3e879da63f84ae87ef628d9ec4d6d0572a77deb0b590636aa93613cec57cde3a74bef"
    "f5c9fa2f081e2bee667ee353b35dc7fd87caa5d7c9460a28efc3adedcfff16970ce22c2d70"
    "5126083843ec3022d10281802481998ac15af000cc3fc0231763f89a1f6fbefd50e2603dac"
    "3d28d9e1c248b4715a81cbf39029012c6717b2062549a8a7e8503d78308fca5d43ed09bf73"
    "3850d89ad8a6d44c3773525358c7d2b1a8f60f7e42d6aa7addbd1e05036f40b11ef81decd5"
    "10b61177ba0fb9e58899f034a7f5067f5cbfbdc3cea7af94a674eab92102818100900556e0"
    "b0fd601a214eecf1047700719df68f8760f18c8d07316d3fe3b329410e23774564be49176c"
    "a9bf5a0f36786c369ed8b8c9e7374eaa53dad942fe180231bb2715c10dbddd120cbe674a33"
    "9165f78f54bd37b2f10ea8ec58adf8de072253559bbc031d8dca2015896e46f0cc145550bf"
    "697d84ac2d7816d4083369bad1028180516f3699897d1c0c103813dbe3438a8f6a8a8ac0b8"
    "540f2f47f51ca3dc9d6b95d3ee2a86de1b6f51af94823e6be1715126f3b2406d96522396c0"
    "6625cdfcf6d4e82ef0ef111dc68969f008313249b63b0ef2921fe98eda58703765ededa3ed"
    "357d390607623cb4329cddf9fe88a957cf49b16c7d14ec2a1e4737e027a526222a"
);

const PUBLIC_KEY_PEM_2048: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEApw1bGRmu4ruORNnfvibq
RIJnZaPBBhY31yc1YF3blk+Nq5sRhCggEcx7eUStBI0rH5i4OLkKr1ZRHJVLTeB0
eOup1Orap3/jy42CTVUEFYda0TNuzfe4H0cbjlcJG+rQDCBVMrVkckHfQhRMzKSy
XmxXmXP3VkO+oDl4QWO9S+f+zvpvMboMCKXsfKr2qPmlYB2kKhjRyTfggWcrNvxs
zDv+sqW24cup6/zkpJ73NNCO14kA8p6u+33ImTbqjyGu2WvjdeKyK1LMimwhHPER
9LJ9sHHYnlVxz3bxd2WrBWw+bTSTh6P/0qZTixMIXWABrOzRHIxDpl5LJPLdIAiX
MQIDAQAB
-----END PUBLIC KEY-----
"#;

const PUBLIC_KEY_DER_2048: &[u8] = &hex!(
    "30820122300d06092a864886f70d01010105000382010f003082010a0282010100a70d5b19"
    "19aee2bb8e44d9dfbe26ea44826765a3c1061637d72735605ddb964f8dab9b1184282011cc"
    "7b7944ad048d2b1f98b838b90aaf56511c954b4de07478eba9d4eadaa77fe3cb8d824d5504"
    "15875ad1336ecdf7b81f471b8e57091bead00c205532b5647241df42144ccca4b25e6c5799"
    "73f75643bea039784163bd4be7fecefa6f31ba0c08a5ec7caaf6a8f9a5601da42a18d1c937"
    "e081672b36fc6ccc3bfeb2a5b6e1cba9ebfce4a49ef734d08ed78900f29eaefb7dc89936ea"
    "8f21aed96be375e2b22b52cc8a6c211cf111f4b27db071d89e5571cf76f17765ab056c3e6d"
    "349387a3ffd2a6538b13085d6001acecd11c8c43a65e4b24f2dd200897310203010001"
);

// Maybe it can be useful to keep the private key for further test ?
#[allow(unused)]
const PRIVATE_KEY_DER_4096: &[u8] = &hex!(
    "30820942020100300d06092a864886f70d01010105000482092c308209280201"
    "000282020100e87bba35288c9afab387c050fbb9576ddd833fbcd1160ab53bf9"
    "5344fbdde082c2f3885eebdf3162b57bde590c4930774bed03fad7ce724ae7b3"
    "7cf0b68600a32f495675048cbfdcc1b6c999fdc8b053b475d257ef275ce7a26c"
    "e347c338d2da8e6caf0f637605c1ea2b8aaca59a4b53e468af363187e360de3c"
    "11cbbcdff20e9813667e8c14051900d8f22e96589262e0072616734662945c97"
    "58f922ef3bed1b8f4a5af5137ac0bada83c77f5b67a2628860e9fd1b67063023"
    "a26e95c14d210a86ad4ff763fee42f9570c17979ff8a78485bb08dda9cdcc5f7"
    "22273e979cf2c8e220969799f6a3400ad23058d45a6f19a79e7a8b4e74cbf791"
    "a26100cb96cf1ad55a5b57ebd238ce5d5e3f0c027f0b7dfa0a04fc2f9bf04a9d"
    "bfb9bfe1ba559cd51c5ba4173880d96dbd8fa46a891835e51caca4f27e5efbed"
    "eca946cf75322f20e75728cff6f3ca16620ce0513f34388f83382757722a453f"
    "736dff8b0f988b1485c274ca9b779ab5f65334e326d544f1782ef1dcd7019d00"
    "2bc93ded1107cff5f90e33e78cce45d6256be59486221188c9732da081110de2"
    "569cc1e6654d54e94372e4b0ba2f227c200dfa8f178b2db6e891868c829c5685"
    "10032c56a9da08fbfee7a22b973ac5f44f91c44bc7596159ef3d6bdf89c4fce0"
    "a777bd9d186bba8f9ceebe1d8e9a709ffb566a5f58d8a8d93ebdc5b5f4d9c984"
    "1627ae0b5583020301000102820200469e06a2cd0d60aa144c80f3587325067f"
    "49b5dba1db432767ef4506e846ae42a9fb158a57b57527d99ea59c80e9de69ee"
    "4b3171498a311765a814a47d9cd8a6b8df5afeb2821a69710217dcc9c4e32299"
    "e74c1c5fcda21fce2bab220a3fc17497dc9594640ede92d791a04eef029e2cfe"
    "6d7a03492a50bc04e5543681c9b89a0c41a059822d369b30b1b566c74e6230dc"
    "81ef64d46125e290f97c123935580b9eda78a88657036b7596987ec9c5b70611"
    "4e01ba31a2d8397df7e508183f1e319223f4931cbf68166209cfb54533f49658"
    "110aa0e785e72dfc8823fcadbb99f5f9650ea70fa7b9ce3ececaa7dae3ae109f"
    "4756113d2d78b6d81d89a8537883f712a475c354468a8567c1d1308e22c936d1"
    "4756573e3236b6d37ee8ee5bf65b166658ea36ede73ab7648298f056bc94ba37"
    "793972701117f92f3def6748d81fed4953b4449dcc6458fab0c26c4161773bfb"
    "ef25e98f21e9302f2b3703a7540a1199603b8ad6a67dd2de714a7d1e4aa5fcb0"
    "e0fb7df9a9e105355da2dfadec4d2e121e08d845c567ec21a9d35c3d1368113c"
    "e1370619ebadd36a6097a4010ba735a3db841757fb3a861698d4a3f44fb4129a"
    "88aa4c3d8733cdead0c288a751b5d44ba1903519878040505e91115d9af15077"
    "31ed26805219600a062f3c80fa40199a4764c9610f494cc4057f81b8bef93966"
    "c5eb4607089e82ccf94976dd32cf910282010100f6f3b806fb948b8499a2a020"
    "e91aba27aab3f9f93b643a51e1dd26a2255c0b81e4138b9d78b40daf2be303c0"
    "c0afdce78a82cbb12da19e22b62fdb6a72adf0e02e4ca22ca217687d115fb40b"
    "8860042417dcfb0385746d53f28e0fd758dabe93081a091cbaa031dc184b745d"
    "899752dd99ec613c5b9e920558b534cf9bf6aaebc3660fa9a835295125424d23"
    "bb27f8ebb192eda62c9bff34f590090e8e7ef844550763d098228700548c6b9a"
    "79fd619b8e48833614f12a7f0bad5b12a6b6082f6833382e75f1466d8dcd0249"
    "32de2c0755b798ec3f0b97e98d59cc88b87cd19302b6a51958663c4e0350eb98"
    "10d2d7c8d04053cd10a64a931dd188d73f6d25f90282010100f1004cac523c1b"
    "2602f8d7e24ba756466a3ee798298eaf29b0e2e7082fa29de81c9bc01839a24d"
    "07ee337aaad9049c48ff64352f0a4557150584e2683f32436616443ee2ec0b0c"
    "dba1c320649914ad617d4fdc5d272c2c95eaa3a06e0c6007151441fd09e70e13"
    "883df616bab8feb0fc0431a4e1b9c6d83753acd66c447c206798b840dc74943c"
    "80fa481664d663cad0be9a4903d6c01218120bba67ef1f758eb3b633cd9c232c"
    "9833a88c6f2b49933c80f336ce529552502861eb5ea7ba1ad825550067634925"
    "5335e0fde35071918dfdc4276cbd8ea5afd2ac461c7a0659b36ec738822867a6"
    "80fb18fab071f2a31587b7cb237fe0ab1e914edb72c8f3065b02820100228ade"
    "afe35ad8d518645dff9c7b87946ad537defbc6be3d9bd9424125f6a5096b2cac"
    "b7bf1d78588b4bfac7686c70fad62e0b6de2131c3a80bf5af29dcde4c686d363"
    "4fd8f06b462b3af6c53233340114716d8f0588ce8e127c7a8cc5b9fb3437cae4"
    "81673d671c012df4bdecb404fc483e7f2f612562096b6a155400ddd4f49b6558"
    "4583e8c3b9bcfb742cce4dfe0c81cf2a7cb6faaf0cea58565bf9e49ab77c2947"
    "75f301c95e6b7524cbfbca4c6fe4cc11c66bd17bff3f53e54c4b9364bbb4d88d"
    "403a712047ccb0e363f7c089ea10bc58a09b04f51fcf0cc386503fed54a1e988"
    "586e3c06ad66db57c8f2797b837455bb2310f421b4485479ce99e99b89028201"
    "01008127b9ef77bac289e279dda86706ecd39bc8ce70db849e16e7b7123d6ed9"
    "e56a293ac6fdb4956856e1af3104327da80beee29325fc89209c21730faaf283"
    "4b5f807b5e805a23a9e66290a1e187b06f2299f79c8f479902e3ecf577dac243"
    "0c489daca7a110f4983f2185aab4b2c3bbb1b3c5af29515861337562611f70ac"
    "5ce9680d06a59ecc7a885c9919773cf60a1148b48280ce2bed067d554fb6b78d"
    "d280818b19c40cf344c8e496657c86644dc5d50d82c79bb66e808ac3bf51e0ed"
    "79c97068576910ea785859bd9412a85fd4b395d5f392f11b6b6c08a94e81a05b"
    "9ae2f0714fb56155fb03908dd87b8af58fb4d0210d7a9ee39691312a63ace648"
    "67950282010049f9be1fa244b6aac2b72d6a348030e5d0ea21ccd1a69122dbb2"
    "163347232c91226a7112895c38ad224c641ff02d24a5f72280825fdd26b09006"
    "b806f29cb57ebdab041c3b50e6ec0bd26142002cc778165937ecfa9fa05c2749"
    "c82cdfa2438e691bdc1296618940a70dabc627ee6ff429d5934ee8706355a956"
    "c2b9fff1ad4dc42cb543c05bf45c12ec30e9ccb7b587f5e47071977d2cbe0c66"
    "4db33a3cde0b01d069043f6213d6c85176b15c1aa879bfce942c2a7d24d64e56"
    "11451790ea4c94cc9bb15c355e6e7f2f8a8f829d804ddd94778fcd5da06ad122"
    "ee3c405cfc4fb91c57fd13992e7a2918fa867a4ef5cd6d4b662c5efd95823a06"
    "5ca424135771"
);

const PUBLIC_KEY_DER_4096: &[u8] = &hex!(
    "30820222300d06092a864886f70d01010105000382020f003082020a02820201"
    "00e87bba35288c9afab387c050fbb9576ddd833fbcd1160ab53bf95344fbdde0"
    "82c2f3885eebdf3162b57bde590c4930774bed03fad7ce724ae7b37cf0b68600"
    "a32f495675048cbfdcc1b6c999fdc8b053b475d257ef275ce7a26ce347c338d2"
    "da8e6caf0f637605c1ea2b8aaca59a4b53e468af363187e360de3c11cbbcdff2"
    "0e9813667e8c14051900d8f22e96589262e0072616734662945c9758f922ef3b"
    "ed1b8f4a5af5137ac0bada83c77f5b67a2628860e9fd1b67063023a26e95c14d"
    "210a86ad4ff763fee42f9570c17979ff8a78485bb08dda9cdcc5f722273e979c"
    "f2c8e220969799f6a3400ad23058d45a6f19a79e7a8b4e74cbf791a26100cb96"
    "cf1ad55a5b57ebd238ce5d5e3f0c027f0b7dfa0a04fc2f9bf04a9dbfb9bfe1ba"
    "559cd51c5ba4173880d96dbd8fa46a891835e51caca4f27e5efbedeca946cf75"
    "322f20e75728cff6f3ca16620ce0513f34388f83382757722a453f736dff8b0f"
    "988b1485c274ca9b779ab5f65334e326d544f1782ef1dcd7019d002bc93ded11"
    "07cff5f90e33e78cce45d6256be59486221188c9732da081110de2569cc1e665"
    "4d54e94372e4b0ba2f227c200dfa8f178b2db6e891868c829c568510032c56a9"
    "da08fbfee7a22b973ac5f44f91c44bc7596159ef3d6bdf89c4fce0a777bd9d18"
    "6bba8f9ceebe1d8e9a709ffb566a5f58d8a8d93ebdc5b5f4d9c9841627ae0b55"
    "830203010001"
);

#[test]
fn only_rsa_is_supported() {
    let unsupported_key: &[u8] = &hex!(
        "308201b73082012b06072a8648ce3804013082011e02818100f8df308877a3bcf66db78734"
        "882f854f934cb5e3bc3cd284b62751d2b6490fd0793dfd43e68393c5fe02f897236fe40949"
        "892aa7718939be9aec22e813c7ab55a245b0b3d2cef69123c862c2449af305ce30784929a3"
        "ba4ac3e5f14cf50508558ee9600c74b5ff1020fe3728ff92c303292dcf13c54aac726ba462"
        "bfee65d3b302150086d038ac40999b7fb3cb7d6253f9ff71a932ca57028180517dea95129a"
        "3e46e7aee51e00fd4eb0bc90eec8c340fe27dc8e72287ebddbdb7c748b67d7416a54b12a1b"
        "d09248d1b6e9291a7d266c02fcf76c887a710065e0fdc767e74dfa13edb5d1a8bca331dd32"
        "dccc199cc2055b446b35a30bd6edde35d08cfbbfd2fbbda2fecce75d4ab5eab2c772cbf914"
        "1325e0e6e05348b035b3ff03818500028181008fb6c7096b0f95aa2baa71e32ba1c1f54088"
        "59f573a4786c55fe050e29180a60b6487885c18a294d1f4655032c1fc3c88eb113dae0cddb"
        "cd5faa40a312dc339256e019aeffd2410c71ef00bf2c519444f3d43024adfc55be6e5db2b7"
        "8e7d6c54c6cc882ffa606e76e53fc73b3443347507e91227944aeed42004a305de09918b");

    assert!(SequesterPublicKeyDer::try_from(unsupported_key).is_err());
    assert!(SequesterVerifyKeyDer::try_from(unsupported_key).is_err());
}

#[rstest]
#[case(PUBLIC_KEY_DER_1024)]
#[case(PUBLIC_KEY_DER_2048)]
fn key_equality(#[case] raw_pub_key: &[u8]) {
    let pub_key = SequesterPublicKeyDer::try_from(raw_pub_key).unwrap();
    let pub_key2 = SequesterPublicKeyDer::try_from(raw_pub_key).unwrap();

    assert_eq!(pub_key, pub_key2);
}

#[rstest]
#[case(SequesterKeySize::_1024Bits)]
#[case(SequesterKeySize::_2048Bits)]
fn size(#[case] size_in_bits: SequesterKeySize) {
    let (priv_key, pub_key) = SequesterPrivateKeyDer::generate_pair(size_in_bits);
    let (signing_key, verify_key) = SequesterSigningKeyDer::generate_pair(size_in_bits);
    assert_eq!(priv_key.size_in_bytes(), size_in_bits as usize / 8);
    assert_eq!(pub_key.size_in_bytes(), size_in_bits as usize / 8);
    assert_eq!(signing_key.size_in_bytes(), size_in_bits as usize / 8);
    assert_eq!(verify_key.size_in_bytes(), size_in_bits as usize / 8);
}

#[rstest]
#[case(SequesterKeySize::_1024Bits)]
#[case(SequesterKeySize::_2048Bits)]
fn encrypt_decrypt(#[case] size_in_bits: SequesterKeySize) {
    let (priv_key, pub_key) = SequesterPrivateKeyDer::generate_pair(size_in_bits);

    let encrypted = pub_key.encrypt(b"foo");

    assert_eq!(priv_key.decrypt(&encrypted).unwrap(), b"foo");
}

#[rstest]
#[case(SequesterKeySize::_1024Bits)]
#[case(SequesterKeySize::_2048Bits)]
fn sign_verify(#[case] size_in_bits: SequesterKeySize) {
    let (signing_key, verify_key) = SequesterSigningKeyDer::generate_pair(size_in_bits);

    let signed = signing_key.sign(b"foo");

    assert_eq!(verify_key.verify(&signed).unwrap(), b"foo");
}

#[rstest]
#[case(
    PRIVATE_KEY_PEM_1024,
    PRIVATE_KEY_DER_1024,
    PUBLIC_KEY_PEM_1024,
    PUBLIC_KEY_DER_1024
)]
#[case(
    PRIVATE_KEY_PEM_2048,
    PRIVATE_KEY_DER_2048,
    PUBLIC_KEY_PEM_2048,
    PUBLIC_KEY_DER_2048
)]
fn import_export(
    #[case] private_key_pem: &str,
    #[case] private_key_der: &[u8],
    #[case] public_key_pem: &str,
    #[case] public_key_der: &[u8],
) {
    let priv_key_pem = SequesterPrivateKeyDer::load_pem(private_key_pem).unwrap();
    let priv_key_der = SequesterPrivateKeyDer::try_from(private_key_der).unwrap();

    let pub_key_pem = SequesterPublicKeyDer::load_pem(public_key_pem).unwrap();
    let pub_key_der = SequesterPublicKeyDer::try_from(public_key_der).unwrap();

    let signing_key_pem = SequesterSigningKeyDer::load_pem(private_key_pem).unwrap();
    let signing_key_der = SequesterSigningKeyDer::try_from(private_key_der).unwrap();

    let verify_key_pem = SequesterVerifyKeyDer::load_pem(public_key_pem).unwrap();
    let verify_key_der = SequesterVerifyKeyDer::try_from(public_key_der).unwrap();

    assert_eq!(priv_key_pem, priv_key_der);
    assert_eq!(pub_key_pem, pub_key_der);

    // Also test roundtrip
    assert_eq!(
        SequesterPrivateKeyDer::load_pem(&priv_key_pem.dump_pem()).unwrap(),
        priv_key_pem
    );
    assert_eq!(
        SequesterPrivateKeyDer::try_from(&priv_key_der.dump()[..]).unwrap(),
        priv_key_der
    );

    assert_eq!(
        SequesterPublicKeyDer::load_pem(&pub_key_pem.dump_pem()).unwrap(),
        pub_key_pem
    );
    assert_eq!(
        SequesterPublicKeyDer::try_from(&pub_key_der.dump()[..]).unwrap(),
        pub_key_der
    );

    assert_eq!(
        SequesterSigningKeyDer::load_pem(&signing_key_pem.dump_pem()).unwrap(),
        signing_key_pem
    );
    assert_eq!(
        SequesterSigningKeyDer::try_from(&signing_key_der.dump()[..]).unwrap(),
        signing_key_der
    );

    assert_eq!(
        SequesterVerifyKeyDer::load_pem(&verify_key_pem.dump_pem()).unwrap(),
        verify_key_pem
    );
    assert_eq!(
        SequesterVerifyKeyDer::try_from(&verify_key_der.dump()[..]).unwrap(),
        verify_key_der
    );
}

#[test]
fn sign_compat() {
    let verify_key = SequesterVerifyKeyDer::try_from(PUBLIC_KEY_DER_1024).unwrap();

    let signed = hex!(
        "5253415353412d5053532d5348413235363a0afc141b03789ac3f2c69bd1e577e279d4570b"
        "f3fe387f389fe52c2b4ac08a9cacbd3c5c1b080cb39969cf3ff7a375619b4b5adc4aef2aec"
        "2800f6ead1d78019f8e37d036880b71e6ba4e89562e14ab2d6b35d0db4db48d818f8d4395f"
        "8d692be38fcdfa8d526a352bb811393dd987ed5a8257b7583d145099037178456baf3c4865"
        "6c6c6f20776f726c64"
    );

    assert_eq!(verify_key.verify(&signed).unwrap(), b"Hello world");
}

#[test]
fn encrypt_compat() {
    let priv_key = SequesterPrivateKeyDer::try_from(PRIVATE_KEY_DER_1024).unwrap();

    let encrypted = hex!(
        "52534145532d4f4145502d5853414c534132302d504f4c59313330353a278b9346743cc609"
        "258a4a82023059411d78c29aed9cf9893dc36b1f1f0055e3db8fa3624b4e3ced4fa1d3683b"
        "97cff2694ddbebeb9a59d1533e9b4dba005958f70a1b7b4b54fef420fb200146a73ac0f457"
        "168e71decb50d98af9332da36b5143e3470f7858a1a43f0f7ffff6e98e2487579d96a3791d"
        "69d48ba307e9984dad42781a567fa27e74e9ee88fd945736968855588eec48f43faa396464"
        "d9a5e8cfd5326ea0193ff5732b51423146d683b74870ed"
    );

    assert_eq!(priv_key.decrypt(&encrypted).unwrap(), b"Hello world");
}

#[test]
fn verify_with_different_salt_len() {
    let verify_key = SequesterVerifyKeyDer::try_from(PUBLIC_KEY_DER_1024).unwrap();
    let ciphered_salt32 = hex!(
        "5253415353412d5053532d5348413235363a" // `RSASSA-PSS-SHA256:` in hex
        "5533ef66fbd8249dcf2f813da9e381bde5d5d730231584e194e43b5a2cd2802a"
        "3a9f713939cf5398f7dd445a97ac9ae7aa3b871b296ff5defa1afe4ee8b1083d"
        "f0c7c2631f0efb400202d5800c0540e87c4c9b94ecc50d99095212baf35e0ea7"
        "9d3a56739867e31126b60d6f11b6448719b69562207a376ca5c5ccc0154318c9"
        "48656c6c6f20576f726c640a" // `Hello world\n` in hex
    );
    let ciphered_salt94 = hex!(
        "5253415353412d5053532d5348413235363a" // `RSASSA-PSS-SHA256:` in hex
        "320bb6b943bdf45fb39bc65b60f6353aa29f11c5990f664b555723cc910a1b07"
        "e95fe82f1ac7c519088d03347461fa7d168686e431f0e4666752ecbcd3d9c576"
        "98cc41b3280784c762effd2771ebc78e55351db499f0242a1e7067e275493d51"
        "3ac8c04fa8f26ca54633c75ec01a9a60eac2eedc74bbf2979a0140e24976e287"
        "48656c6c6f20576f726c640a" // `Hello world\n` in hex
    );
    let data = b"Hello World\n";

    assert_eq!(
        verify_key
            .verify(&ciphered_salt32)
            .expect("Cannot verify salt32"),
        data
    );
    assert_eq!(
        verify_key
            .verify(&ciphered_salt94)
            .expect("Cannot verify salt94"),
        data
    );
}

#[rstest]
#[case(SequesterKeySize::_1024Bits)]
#[case(SequesterKeySize::_2048Bits)]
fn sign_unsecure_unwrap(#[case] size_in_bits: SequesterKeySize) {
    let (signing_key, _verify_key) = SequesterSigningKeyDer::generate_pair(size_in_bits);

    let signed = signing_key.sign(b"foo");

    let (_, data) = SequesterVerifyKeyDer::unsecure_unwrap(&signed).unwrap();

    assert_eq!(data, b"foo");
}

#[test]
fn sign_compat_4096_without_size() {
    let verify_key = SequesterVerifyKeyDer::try_from(PUBLIC_KEY_DER_4096).unwrap();

    let data = b"Hello world";

    let signed = hex!(
        "5253415353412d5053532d5348413235363ae3e75e17fa9eb458997d42a4ca875ae1448ee8"
        "90ad851c43d39e77135237850b8b15a2f1dd164c8a2052ec983c164e673c6ad97be19d4e3a"
        "a9a2a7ae7a69921f235b253f5a4c4b98eea052e9addec763f1b75d17a522dd8d5dc4247d2e"
        "92d2c0bb65970da272d0433e864454b764877e6175e4efd7900c0c16868db4b825d799ad79"
        "b4e3346f10ea5566af54300717d32f684142e0057527ae7fc8f4816002223fe085f65b7de4"
        "3f5ed7ff3b7a4b1a71025cf987ffffc7d0705dbc97ec7c5702635ea32613a5e01e50cbcb02"
        "cae233ccf328ae1c42fdacde88ae7c24ac811e1846396bd16211addc52a0526515f22b9517"
        "d464aaf9b7ec7f3d75555d686fdd2c6e58d9c3b3e5d408373da90db011e6e44f9905097fee"
        "a10342728e81408e05fe784aa0f8549963d2a4ca8b138c4d67b65031434cd020c35d791a60"
        "c52fb740093264bfb1f989dc2214ba86a9fca98f6ec42eb62fcba384b1a7a9940f1249e923"
        "185920b244f4398112bc626256481ce575434133914be1e084d5e8c15c922a1269d83bf050"
        "83d0b35d85fc88eb1dc4935554dfdd174879cb801913799b18d1269efe4649dcb3bb132e69"
        "7a6dae981f77557d2d23a8ce279fb7029a7da0c88f1c00f5bda5c78f52e95283997ea927b0"
        "a2c38b033118017b092f470c198ac4762f4564ecd12e474310141282a3b352a20d8a3c59b2"
        "c653d00c9666ecd9c275232848656c6c6f20776f726c64"
    );

    assert_eq!(verify_key.verify(&signed).unwrap(), data);

    let (signature, output) = SequesterVerifyKeyDer::unsecure_unwrap(&signed).unwrap();

    assert!(signature.starts_with(b"RSASSA-PSS-SHA256:"));
    assert_eq!(output, data);
}

#[test]
fn sign_compat_4096_with_size() {
    let verify_key = SequesterVerifyKeyDer::try_from(PUBLIC_KEY_DER_4096).unwrap();

    let data = b"Hello world";

    let signed = hex!(
        "5253415353412d5053532d5348413235364000023ae3e75e17fa9eb458997d42a4ca875ae1"
        "448ee890ad851c43d39e77135237850b8b15a2f1dd164c8a2052ec983c164e673c6ad97be1"
        "9d4e3aa9a2a7ae7a69921f235b253f5a4c4b98eea052e9addec763f1b75d17a522dd8d5dc4"
        "247d2e92d2c0bb65970da272d0433e864454b764877e6175e4efd7900c0c16868db4b825d7"
        "99ad79b4e3346f10ea5566af54300717d32f684142e0057527ae7fc8f4816002223fe085f6"
        "5b7de43f5ed7ff3b7a4b1a71025cf987ffffc7d0705dbc97ec7c5702635ea32613a5e01e50"
        "cbcb02cae233ccf328ae1c42fdacde88ae7c24ac811e1846396bd16211addc52a0526515f2"
        "2b9517d464aaf9b7ec7f3d75555d686fdd2c6e58d9c3b3e5d408373da90db011e6e44f9905"
        "097feea10342728e81408e05fe784aa0f8549963d2a4ca8b138c4d67b65031434cd020c35d"
        "791a60c52fb740093264bfb1f989dc2214ba86a9fca98f6ec42eb62fcba384b1a7a9940f12"
        "49e923185920b244f4398112bc626256481ce575434133914be1e084d5e8c15c922a1269d8"
        "3bf05083d0b35d85fc88eb1dc4935554dfdd174879cb801913799b18d1269efe4649dcb3bb"
        "132e697a6dae981f77557d2d23a8ce279fb7029a7da0c88f1c00f5bda5c78f52e95283997e"
        "a927b0a2c38b033118017b092f470c198ac4762f4564ecd12e474310141282a3b352a20d8a"
        "3c59b2c653d00c9666ecd9c275232848656c6c6f20776f726c64"
    );

    assert_eq!(verify_key.verify(&signed).unwrap(), data);

    let (signature, output) = SequesterVerifyKeyDer::unsecure_unwrap(&signed).unwrap();

    assert!(signature.starts_with(b"RSASSA-PSS-SHA256@"));
    assert_eq!(output, data);
}

#[test]
fn invalid_signature_with_correct_size_and_data_with_correct_size() {
    let verify_key = SequesterVerifyKeyDer::try_from(PUBLIC_KEY_DER_1024).unwrap();
    let signed = [
        &b"RSASSA-PSS-SHA256@"[..],
        &128u16.to_le_bytes()[..],
        &[b':'][..],
        &[0; 128][..],
    ]
    .concat();

    let e = verify_key.verify(&signed).unwrap_err();

    assert_eq!(e, CryptoError::SignatureVerification);

    // This is unsecure, the signature is invalid but the length is correct
    let (_signature, data) = SequesterVerifyKeyDer::unsecure_unwrap(&signed).unwrap();

    assert_eq!(data, [])
}

#[test]
fn invalid_signature_without_size_but_long_data() {
    let verify_key = SequesterVerifyKeyDer::try_from(PUBLIC_KEY_DER_1024).unwrap();
    let signed = [0; 1024];

    let e = verify_key.verify(&signed).unwrap_err();

    assert_eq!(e, CryptoError::Decryption);

    // This is unsecure, the signature is invalid but the length is correct
    let (_signature, data) = SequesterVerifyKeyDer::unsecure_unwrap(&signed).unwrap();

    assert_eq!(data, [0; 494])
}

#[rstest]
#[case::empty(b"", CryptoError::Decryption)]
#[case::only_separator(b":", CryptoError::Algorithm("".into()))]
#[case::no_signature(b"RSASSA-PSS-SHA256:", CryptoError::DataSize)]
#[case::signature_too_small(b"RSASSA-PSS-SHA256:\0", CryptoError::DataSize)]
#[case::missing_algorithm(b"@\0\0:", CryptoError::Algorithm("".into()))]
#[case::missing_separator(b"RSASSA-PSS-SHA256@\0\0", CryptoError::Decryption)]
#[case::missing_size(b"RSASSA-PSS-SHA256@:", CryptoError::Algorithm("RSASSA-PSS-SHA256@".into()))]
#[case::signature_with_correct_size_but_invalid_data(
    b"RSASSA-PSS-SHA256@\x80\0:",
    CryptoError::DataSize
)]
// Compatibility test: When size is missing in signature, the default RSA key' size is 4096 bits
#[case::unsecure_unwrap_is_invalid_if_signature_is_too_short(&[0; 529][..], CryptoError::Decryption)]
fn invalid_signature(#[case] signed: &[u8], #[case] err: CryptoError) {
    let verify_key = SequesterVerifyKeyDer::try_from(PUBLIC_KEY_DER_1024).unwrap();

    let e = verify_key.verify(signed).unwrap_err();

    assert_eq!(e, err);

    let e = SequesterVerifyKeyDer::unsecure_unwrap(signed).unwrap_err();

    assert_eq!(e, CryptoError::Signature);
}
