// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use recorded_tests_macros::recorded;

    #[recorded]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
