use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

const RENAMED_RULES: &[(&str, &str, &str)] = &[
    (
        "generate_keyword",
        "keyword_forbidden_generate",
        "KeywordForbiddenGenerate",
    ),
    ("tab_charactor", "tab_character", "TabCharacter"),
    (
        "genvar_declaration",
        "genvar_declaration_in_loop",
        "GenvarDeclarationInLoop",
    ),
    ("if_with_begin", "multiline_if_begin", "MultilineIfBegin"),
    ("for_with_begin", "multiline_for_begin", "MultilineForBegin"),
    (
        "legacy_always",
        "keyword_forbidden_always",
        "KeywordForbiddenAlways",
    ),
    (
        "generate_keyword_forbidden",
        "keyword_forbidden_generate",
        "KeywordForbiddenGenerate",
    ),
    (
        "priority_keyword",
        "keyword_forbidden_priority",
        "KeywordForbiddenPriority",
    ),
    (
        "unique_keyword",
        "keyword_forbidden_unique",
        "KeywordForbiddenUnique",
    ),
    (
        "unique0_keyword",
        "keyword_forbidden_unique0",
        "KeywordForbiddenUnique0",
    ),
    (
        "wire_reg",
        "keyword_forbidden_wire_reg",
        "KeywordForbiddenWireReg",
    ),
    (
        "generate_keyword_required",
        "keyword_required_generate",
        "KeywordRequiredGenerate",
    ),
];

fn write_rules_rs(rules: &Vec<(String, String)>) -> () {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let o = Path::new(&out_dir).join("rules.rs");
    let mut o = File::create(&o).unwrap();

    for (rulename, _) in rules {
        let _ = writeln!(
            o,
            "#[path = \"{}/src/rules/{}.rs\"]",
            root_dir.replace("\\", "\\\\"),
            rulename
        );
        let _ = writeln!(o, "pub mod {};", rulename);
    }

    for (rulename, _) in rules {
        let _ = writeln!(o, "pub use {}::*;", rulename);
    }
}

fn write_config_rules_rs(rules: &Vec<(String, String)>) -> () {
    let out_dir = env::var("OUT_DIR").unwrap();
    let o = Path::new(&out_dir).join("config_rules.rs");
    let mut o = File::create(&o).unwrap();

    let _ = writeln!(o, "");
    let _ = writeln!(o, "#[derive(Clone, Debug, Deserialize, Serialize)]");
    let _ = writeln!(o, "#[serde(deny_unknown_fields)]");
    let _ = writeln!(o, "pub struct ConfigRules {{");

    for (rulename, _) in rules {
        let _ = writeln!(o, "    #[serde(default = \"default_as_false\")]");
        let _ = writeln!(o, "    pub {}: bool,", rulename);
    }

    let _ = writeln!(o, "");

    for (original_rulename, _, _) in RENAMED_RULES {
        let _ = writeln!(o, "    #[serde(default = \"default_as_false\", skip_serializing)]");
        let _ = writeln!(o, "    pub {}: bool,", original_rulename);
    }

    let _ = writeln!(o, "}}");
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let re_struct = Regex::new(r"pub struct ([a-zA-Z0-9]*)").unwrap();

    let mut rules = Vec::new();
    for entry in WalkDir::new("src/rules") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let f = File::open(entry.path()).unwrap();
            let mut f = BufReader::new(f);
            let mut s = String::new();
            let _ = f.read_to_string(&mut s);
            let cap = re_struct.captures(&s);
            if let Some(cap) = cap {
                let struct_name = String::from(&cap[1]);
                let file_name = String::from(entry.path().file_stem().unwrap().to_string_lossy());
                rules.push((file_name, struct_name));
            }
        }
    }

    rules.sort_by(|a, b| a.0.cmp(&b.0));

    write_rules_rs(&rules);
    write_config_rules_rs(&rules);

    // -------------------------------------------------------------------------------------------------
    // Output 'impl_config.rs'
    // -------------------------------------------------------------------------------------------------

    let out_impl_config = Path::new(&out_dir).join("impl_config.rs");
    let mut out_impl_config = File::create(&out_impl_config).unwrap();

    let mut enable_all_body = String::new();
    let mut gen_rules_body = String::new();
    let mut gen_all_rules_body = String::new();
    let mut check_rename_body = String::new();
    let mut migrate_body = String::new();
    for (file_name, struct_name) in &rules {
        enable_all_body.push_str(&format!("        self.rules.{} = true;\n", file_name));
        gen_rules_body.push_str(&format!("        if self.rules.{} {{\n", file_name));
        gen_rules_body.push_str(&format!(
            "            ret.push(Box::new({}::default()));\n",
            struct_name
        ));
        gen_rules_body.push_str(&format!("        }}\n"));
        gen_all_rules_body.push_str(&format!(
            "        ret.push(Box::new({}::default()));\n",
            struct_name
        ));
    }
    for (org_name, file_name, struct_name) in RENAMED_RULES {
        gen_rules_body.push_str(&format!("        if self.rules.{} {{\n", org_name));
        gen_rules_body.push_str(&format!(
            "            ret.push(Box::new({}::default()));\n",
            struct_name
        ));
        gen_rules_body.push_str(&format!("        }}\n"));
        check_rename_body.push_str(&format!("        if self.rules.{} {{\n", org_name));
        check_rename_body.push_str(&format!(
            "            ret.push((String::from(\"{}\"), String::from(\"{}\")));\n",
            org_name, file_name
        ));
        check_rename_body.push_str(&format!("        }}\n"));
        migrate_body.push_str(&format!(
            "        self.rules.{} = self.rules.{};\n",
            file_name, org_name
        ));
    }

    let str_impl_config = format!(
        r##"
impl Config {{
    pub fn new() -> Self {{
        toml::from_str("").unwrap()
    }}

    pub fn enable_all(mut self) -> Self {{
{}
        self
    }}

    pub fn gen_rules(&self) -> Vec<Box<dyn Rule>> {{
        let mut ret: Vec<Box<dyn Rule>> = Vec::new();
{}
        ret
    }}

    pub fn gen_all_rules() -> Vec<Box<dyn Rule>> {{
        let mut ret: Vec<Box<dyn Rule>> = Vec::new();
{}
        ret
    }}

    pub fn check_rename(&self) -> Vec<(String, String)> {{
        let mut ret: Vec<(String, String)> = Vec::new();
{}
        ret
    }}

    pub fn migrate(&mut self) {{
{}
    }}
}}"##,
        enable_all_body, gen_rules_body, gen_all_rules_body, check_rename_body, migrate_body
    );
    let _ = write!(out_impl_config, "{}", str_impl_config);

    // -------------------------------------------------------------------------------------------------
    // Output 'test.rs'
    // -------------------------------------------------------------------------------------------------

    let out_test = Path::new(&out_dir).join("test.rs");
    let mut out_test = File::create(&out_test).unwrap();

    // blocking_assignment_in_always_ff testcases demonstrate comment control,
    // so visibility is useful when running `cargo test`.
    let test_verbose = ["blocking_assignment_in_always_ff"];

    for (rulename, _) in &rules {
        let silent = if test_verbose.contains(&rulename.as_str()) {
            "false"
        } else {
            "true"
        };

        for pass_not_fail in [true, false].iter() {
            let passfail = if *pass_not_fail {
                "pass"
            } else {
                "fail"
            };

            let test_filename = format!("testcases/{}/{}.sv", passfail, rulename);
            let lines = BufReader::new(File::open(test_filename).unwrap())
                .lines()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            let sep = "/".repeat(80);
            let testcases: Vec<&[String]> = lines
                .as_slice()
                .split(|l| l.contains(sep.as_str()))
                .collect();
            let n_testcases: usize = testcases.len();

            for (t, testcase) in testcases
                .into_iter()
                .enumerate()
                .map(|(i, x)| (i + 1, x)) {
                // Write subtest to its own file.
                let subtest_path = Path::new(&out_dir)
                    .join(format!("{rulename}.{passfail}.{t}of{n_testcases}.sv"));
                let mut out_subtest = File::create(&subtest_path).unwrap();
                for line in testcase {
                    let _ = write!(out_subtest, "{}\n", line);
                }

                // Create call to `main.rs::tests::test()` via `tests.rs`.
                let subtest_name = format!("{rulename}_{passfail}_{t}of{n_testcases}");
                let _ = write!(out_test, "#[test]\n");
                let _ = write!(out_test, "fn {}() {{\n", subtest_name);
                if *pass_not_fail {
                    let _ = write!(out_test, "    test(\"{rulename}\", {subtest_path:?}, true, {silent}, false);\n");
                } else {
                    let _ = write!(out_test, "    test(\"{rulename}\", {subtest_path:?}, false, {silent}, false);\n");
                    let _ = write!(out_test, "    test(\"{rulename}\", {subtest_path:?}, false, {silent}, true);\n");
                }
                let _ = write!(out_test, "}}\n");

            }
        }
    }
}
