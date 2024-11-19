// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use recorded_tests_core::{TestContext, TestMode};
    use recorded_tests_macros::recorded;

    #[recorded]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[recorded(live)]
    async fn it_works_async(ctx: TestContext) {
        assert_eq!(ctx.test_mode(), TestMode::Live);
        assert_eq!(ctx.test_name(), "it_works_async");

        let mut i = 0;
        tokio::spawn(async move {
            i = add(i, 1);
            assert_eq!(i, 1);
        })
        .await
        .unwrap();

        assert_eq!(i, 0);
    }
}
