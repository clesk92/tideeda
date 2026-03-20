//! UHDM 回归测试系统

use capnp::serialize_packed;
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use crate::ReaderOptions;
use crate::{UhdmDesign, UhdmContext, UhdmError};
use crate::walker::{walk_design, walk_all, walk_design_tree};

#[derive(Debug, Clone)]
pub enum TestResult {
    Pass,
    Fail { diff: String },
    Error(String),
    MissingGolden { containers: bool, design: bool, walker: bool },
}

#[derive(Debug)]
pub struct TestCase {
    pub name: String,
    pub test_dir: PathBuf,
}

impl TestCase {
    pub fn uhdm_path(&self) -> PathBuf {
        self.test_dir.join(format!("{}.uhdm", self.name))
    }

    pub fn golden_dir(&self) -> PathBuf { self.test_dir.join("golden") }
    pub fn actual_dir(&self) -> PathBuf { self.test_dir.join("actual") }

    pub fn containers_golden_path(&self) -> PathBuf { self.golden_dir().join("containers.golden.log") }
    pub fn design_golden_path(&self) -> PathBuf { self.golden_dir().join("design.golden.log") }
    pub fn walker_golden_path(&self) -> PathBuf { self.golden_dir().join("walker.golden.log") }
    pub fn tree_golden_path(&self) -> PathBuf { self.golden_dir().join("tree.golden.log") }

    pub fn containers_actual_path(&self) -> PathBuf { self.actual_dir().join("containers.actual.log") }
    pub fn design_actual_path(&self) -> PathBuf { self.actual_dir().join("design.actual.log") }
    pub fn walker_actual_path(&self) -> PathBuf { self.actual_dir().join("walker.actual.log") }
    pub fn tree_actual_path(&self) -> PathBuf { self.actual_dir().join("tree.actual.log") }
}

pub struct RegressionTester {
    regression_dir: PathBuf,
    results: HashMap<String, TestResult>,
}

impl RegressionTester {
    pub fn new(regression_dir: &str) -> Self {
        Self {
            regression_dir: PathBuf::from(regression_dir),
            results: HashMap::new(),
        }
    }

    pub fn discover_tests(&self) -> Vec<TestCase> {
        let mut tests = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.regression_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown").to_string();
                    let uhdm_path = path.join(format!("{}.uhdm", name));
                    if uhdm_path.exists() {
                        tests.push(TestCase { name, test_dir: path });
                    }
                }
            }
        }
        tests
    }

    pub fn run_test(&self, test: &TestCase) -> TestResult {
        if !test.uhdm_path().exists() {
            return TestResult::Error("UHDM file not found".to_string());
        }

        let containers_golden_exists = test.containers_golden_path().exists();
        let design_golden_exists = test.design_golden_path().exists();
        let walker_golden_exists = test.walker_golden_path().exists();
        let tree_golden_exists = test.tree_golden_path().exists();

        if !containers_golden_exists || !design_golden_exists || !walker_golden_exists || !tree_golden_exists {
            return TestResult::MissingGolden {
                containers: containers_golden_exists,
                design: design_golden_exists,
                walker: walker_golden_exists,
            };
        }

        let (containers_output, design_output, walker_output, tree_output) = match self.generate_outputs(&test.uhdm_path()) {
            Ok(o) => o,
            Err(e) => return TestResult::Error(format!("{:?}", e)),
        };

        let _ = fs::create_dir_all(test.actual_dir());
        let _ = fs::write(test.containers_actual_path(), &containers_output);
        let _ = fs::write(test.design_actual_path(), &design_output);
        let _ = fs::write(test.walker_actual_path(), &walker_output);
        let _ = fs::write(test.tree_actual_path(), &tree_output);

        let containers_expected = fs::read_to_string(test.containers_golden_path()).ok();
        let design_expected = fs::read_to_string(test.design_golden_path()).ok();
        let walker_expected = fs::read_to_string(test.walker_golden_path()).ok();
        let tree_expected = fs::read_to_string(test.tree_golden_path()).ok();

        let containers_match = containers_expected.as_ref().map(|e| Self::compare(e, &containers_output)).unwrap_or(false);
        let design_match = design_expected.as_ref().map(|e| Self::compare(e, &design_output)).unwrap_or(false);
        let walker_match = walker_expected.as_ref().map(|e| Self::compare(e, &walker_output)).unwrap_or(false);
        let tree_match = tree_expected.as_ref().map(|e| Self::compare(e, &tree_output)).unwrap_or(false);

        if containers_match && design_match && walker_match && tree_match {
            TestResult::Pass
        } else {
            let diff = format!("containers: {}, design: {}, walker: {}, tree: {}", containers_match, design_match, walker_match, tree_match);
            TestResult::Fail { diff }
        }
    }

    pub fn run_all_tests(&mut self) -> &HashMap<String, TestResult> {
        for test in &self.discover_tests() {
            println!("Running test: {}", test.name);
            let result = self.run_test(test);
            self.results.insert(test.name.clone(), result);
        }
        &self.results
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        let total = self.results.len();
        let passed = self.results.values().filter(|r| matches!(r, TestResult::Pass)).count();
        let failed = self.results.values().filter(|r| matches!(r, TestResult::Fail { .. })).count();

        report.push_str("========================================\n");
        report.push_str("       UHDM 回归测试报告\n");
        report.push_str("========================================\n\n");
        report.push_str(&format!("总计: {} | 通过: {} | 失败: {}\n\n", total, passed, failed));

        for (name, result) in &self.results {
            match result {
                TestResult::Pass => report.push_str(&format!("[PASS] {}\n", name)),
                TestResult::Fail { diff } => report.push_str(&format!("[FAIL] {}: {}\n", name, diff)),
                TestResult::Error(msg) => report.push_str(&format!("[ERROR] {}: {}\n", name, msg)),
                TestResult::MissingGolden { .. } => report.push_str(&format!("[MISSING] {}\n", name)),
            }
        }
        report
    }

    fn generate_outputs(&self, uhdm_path: &Path) -> Result<(String, String, String, String), UhdmError> {
        let file = std::fs::File::open(uhdm_path)?;
        let mut reader = BufReader::new(file);
        let message_reader = serialize_packed::read_message(&mut reader, ReaderOptions::new())
            .map_err(|_e| UhdmError::InvalidFormat)?;
        let root = message_reader
            .get_root::<crate::uhdm_capnp::uhdm_root::Reader>()
            .map_err(|e| UhdmError::RootNodeError(e.to_string()))?;
        let ctx = UhdmContext::new(root);

        let design = UhdmDesign::load(uhdm_path.to_str().unwrap())?;
        let containers = design.format_containers();
        let design_str = design.format_design();
        let walker_output = walk_all(&ctx);
        let tree_output = walk_design_tree(&ctx);
        Ok((containers, design_str, walker_output, tree_output))
    }

    fn compare(expected: &str, actual: &str) -> bool {
        let exp: Vec<_> = expected.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
        let act: Vec<_> = actual.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
        exp == act
    }

    pub fn generate_golden(&self, test: &TestCase) -> Result<(), UhdmError> {
        let (containers, design, walker, tree) = self.generate_outputs(&test.uhdm_path())?;

        fs::create_dir_all(test.golden_dir()).map_err(UhdmError::Io)?;
        fs::write(test.containers_golden_path(), containers).map_err(UhdmError::Io)?;
        fs::write(test.design_golden_path(), design).map_err(UhdmError::Io)?;
        fs::write(test.walker_golden_path(), walker).map_err(UhdmError::Io)?;
        fs::write(test.tree_golden_path(), tree).map_err(UhdmError::Io)?;

        println!("Generated golden logs for: {}", test.name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regression_counter() {
        let tester = RegressionTester::new("/Users/hh/Documents/TideEDA/tests/examples");

        let test = TestCase {
            name: "counter".to_string(),
            test_dir: PathBuf::from("/Users/hh/Documents/TideEDA/tests/examples/counter"),
        };

        if !test.uhdm_path().exists() {
            println!("SKIP: counter test - UHDM file not found at {:?}", test.uhdm_path());
            return;
        }

        let needs_golden = !test.containers_golden_path().exists()
            || !test.design_golden_path().exists()
            || !test.walker_golden_path().exists()
            || !test.tree_golden_path().exists();

        if needs_golden {
            println!("Generating golden logs...");
            tester.generate_golden(&test).expect("generate golden");
        }

        let result = tester.run_test(&test);

        match &result {
            TestResult::Pass => println!("PASSED: {}", test.name),
            TestResult::Fail { diff } => println!("FAILED: {} - {}", test.name, diff),
            TestResult::Error(msg) => println!("ERROR: {} - {}", test.name, msg),
            TestResult::MissingGolden { .. } => println!("MISSING: {}", test.name),
        }

        assert!(matches!(result, TestResult::Pass));
    }

    #[test]
    fn test_regression_hierarchy() {
        let tester = RegressionTester::new("/Users/hh/Documents/TideEDA/tests/examples");

        let test = TestCase {
            name: "hierarchy".to_string(),
            test_dir: PathBuf::from("/Users/hh/Documents/TideEDA/tests/examples/hierarchy"),
        };

        if !test.uhdm_path().exists() {
            println!("SKIP: hierarchy test - UHDM file not found at {:?}", test.uhdm_path());
            return;
        }

        let needs_golden = !test.containers_golden_path().exists()
            || !test.design_golden_path().exists()
            || !test.walker_golden_path().exists()
            || !test.tree_golden_path().exists();

        if needs_golden {
            println!("Generating golden logs for hierarchy...");
            tester.generate_golden(&test).expect("generate golden");
        }

        let result = tester.run_test(&test);

        match &result {
            TestResult::Pass => println!("PASSED: {}", test.name),
            TestResult::Fail { diff } => println!("FAILED: {} - {}", test.name, diff),
            TestResult::Error(msg) => println!("ERROR: {} - {}", test.name, msg),
            TestResult::MissingGolden { .. } => println!("MISSING: {}", test.name),
        }

        assert!(matches!(result, TestResult::Pass));
    }

    #[test]
    fn test_regression_ibex_csr() {
        let tester = RegressionTester::new("/Users/hh/Documents/TideEDA/tests/examples");

        let test = TestCase {
            name: "ibex_csr".to_string(),
            test_dir: PathBuf::from("/Users/hh/Documents/TideEDA/tests/examples/ibex_csr"),
        };

        if !test.uhdm_path().exists() {
            println!("SKIP: ibex_csr test - UHDM file not found at {:?}", test.uhdm_path());
            return;
        }

        let needs_golden = !test.containers_golden_path().exists()
            || !test.design_golden_path().exists()
            || !test.walker_golden_path().exists()
            || !test.tree_golden_path().exists();

        if needs_golden {
            println!("Generating golden logs for ibex_csr...");
            tester.generate_golden(&test).expect("generate golden");
        }

        let result = tester.run_test(&test);

        match &result {
            TestResult::Pass => println!("PASSED: {}", test.name),
            TestResult::Fail { diff } => println!("FAILED: {} - {}", test.name, diff),
            TestResult::Error(msg) => println!("ERROR: {} - {}", test.name, msg),
            TestResult::MissingGolden { .. } => println!("MISSING: {}", test.name),
        }

        assert!(matches!(result, TestResult::Pass));
    }
}
