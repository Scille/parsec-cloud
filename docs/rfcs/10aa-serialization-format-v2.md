<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

A bit of benchmarking:

[see script](./10aa-serialization-format-v2/bench_user_certif.py)

full_patate (no deflate):
    size: 135o 55.33%
    serialize: 2.08us 15.39%
    deserialize: 1.85us 29.76%

full_patate (deflate):
    size: 146o 59.84%
    serialize: 11.71us 86.85%
    deserialize: 2.60us 41.89%

indexed_keys (deflate):
    size: 184o 75.41%
    serialize: 11.95us 88.64%
    deserialize: 4.84us 77.93%

indexed_keys (no deflate):
    size: 206o 84.43%
    serialize: 1.96us 14.52%
    deserialize: 3.96us 63.78%

uuid_when_possible (no deflate):
    size: 215o 88.11%
    serialize: 2.19us 16.28%
    deserialize: 5.07us 81.62%

indexed_enums (deflate):
    size: 221o 90.57%
    serialize: 13.80us 102.40%
    deserialize: 5.85us 94.20%

uuid_when_possible (deflate):
    size: 222o 90.98%
    serialize: 14.38us 106.64%
    deserialize: 6.03us 97.15%

indexed_discriminant (deflate):
    size: 230o 94.26%
    serialize: 12.94us 95.97%
    deserialize: 6.24us 100.47%

timestamp_as_int_of_us (deflate):
    size: 242o 99.18%
    serialize: 13.41us 99.52%
    deserialize: 4.75us 76.56%

default (deflate):
    size: 244o
    serialize: 13.48us
    deserialize: 6.21us

indexed_enums (no deflate):
    size: 245o 100.41%
    serialize: 1.97us 14.58%
    deserialize: 4.24us 68.36%

indexed_discriminant (no deflate):
    size: 253o 103.69%
    serialize: 1.94us 14.39%
    deserialize: 4.23us 68.12%

timestamp_as_int_of_us (no deflate):
    size: 268o 109.84%
    serialize: 1.85us 13.75%
    deserialize: 2.84us 45.75%

default (no deflate):
    size: 269o 110.25%
    serialize: 1.91us 14.19%
    deserialize: 4.34us 69.86%
