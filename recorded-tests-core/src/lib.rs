// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::{
    env, fmt,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Clone, Debug, Default)]
pub struct TestContext {
    test_mode: TestMode,
    test_name: &'static str,
    recordings_dir: PathBuf,
}

impl TestContext {
    pub fn new(test_mode: TestMode, module_path: &'static str, test_name: &'static str) -> Self {
        let mut recordings_dir =
            PathBuf::from_str(&env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
                .unwrap();
        module_path
            .split("::")
            .skip(1)
            .for_each(|path| recordings_dir.push(path));
        recordings_dir.push("data");

        Self {
            test_mode,
            test_name,
            recordings_dir,
        }
    }

    pub fn test_mode(&self) -> TestMode {
        self.test_mode
    }

    pub fn test_name(&self) -> &'static str {
        self.test_name
    }

    pub fn recordings_dir(&self) -> &Path {
        self.recordings_dir.as_path()
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestMode {
    #[default]
    Playback,
    Record,
    Live,
}

impl fmt::Debug for TestMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str((*self).into())
    }
}

impl From<TestMode> for &str {
    fn from(mode: TestMode) -> Self {
        match mode {
            TestMode::Playback => "playback",
            TestMode::Record => "record",
            TestMode::Live => "live",
        }
    }
}
impl FromStr for TestMode {
    type Err = ParseTestModeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "playback" => Ok(Self::Playback),
            "record" => Ok(Self::Record),
            "live" => Ok(Self::Live),
            _ => Err(ParseTestModeError),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTestModeError;

impl fmt::Display for ParseTestModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("provided string was not 'playback', 'record', or 'live'")
    }
}

impl std::error::Error for ParseTestModeError {}
