// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// The drive letters that parsec can use for mounting workspaces, sorted by priority.
// For instance, if a device has 3 workspaces, they will preferably be mounted on
// `P:\`, `Q:\` and `R:\` respectively. Make sure its length is a prime number so it
// plays well with the algorithm in `sorted_drive_letters`.
const DRIVE_LETTERS: [u8; 19] = *b"PQRSTUVWXYZHIJKLMNO";

/// Sort the drive letters for a specific workspace index, by decreasing priority.
///
/// The first letter should preferably be used. If it's not available, then the second
/// letter should be used and so on.
///
/// The number of workspaces (`length`) is also important as this algorithm will round
/// it to the next multiple of 5 and use it to group the workspaces together.
///
/// Example with 3 workspaces (rounded to 5 slots)
///
/// | Candidates | W1 | W2 | W3 | XX | XX |
/// |------------|----|----|----|----|----|
/// | 0          | P  | Q  | R  | S  | T  |
/// | 1          | U  | V  | W  | X  | Y  |
/// | 2          | Z  | H  | I  | J  | K  |
/// | 3          | L  | M  | N  | O  | P  |
/// | 4          | Q  | R  | S  | T  | U  |
/// | ...
pub(crate) fn sorted_drive_letters(index: usize, length: usize) -> impl Iterator<Item = char> {
    const GROUPING: usize = 5;
    assert!(index < length);

    // Round to the next multiple of grouping, i.e 5
    let length = match length % GROUPING {
        0 => length,
        _ => ((length / GROUPING) + 1) * GROUPING,
    };

    let gcd = |mut a: usize, mut b: usize| {
        // Compute the greatest common divisor
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    };
    // For the algorithm to work well, the lengths should be coprimes
    assert!(gcd(length, DRIVE_LETTERS.len()) == 1);

    // Get all the letters by circling around the drive letter list
    let mut index = index;
    (0..DRIVE_LETTERS.len()).map(move |_| {
        let letter = DRIVE_LETTERS[index % DRIVE_LETTERS.len()];
        index += length;
        char::from(letter)
    })
}

#[cfg(test)]
#[path = "../../tests/unit/windows_drive_letter.rs"]
mod tests;
