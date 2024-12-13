use crate::utils::execute_run_script;
use crate::{error::Error, RuleCheck, TEMPLATES};
use regex::Regex;
use serde::Serialize;
use std::fs;
use std::io::{self, BufRead};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tera::Context;

pub struct DrcParams<'a> {
    pub cell_name: &'a str,
    pub work_dir: &'a Path,
    pub layout_path: &'a Path,
    pub rules_path: &'a Path,
}

pub struct DrcGeneratedPaths {
    pub runset_path: PathBuf,
    pub summary_path: PathBuf,
}

#[derive(Serialize)]
struct DrcRunsetContext<'a> {
    work_dir: &'a Path,
    layout_path: &'a Path,
    cell_name: &'a str,
    results_path: &'a Path,
    summary_path: &'a Path,
}

pub struct DrcData {
    pub rule_checks: Vec<RuleCheck>,
}

pub fn write_drc_files(params: &DrcParams) -> Result<DrcGeneratedPaths, Error> {
    fs::create_dir_all(params.work_dir).map_err(Error::Io)?;

    let results_path = params.work_dir.join("drc.results");
    let summary_path = params.work_dir.join("drc.summary");
    let runset_path = params.work_dir.join("pegasusdrcctl");
    let view_drc_path = params.work_dir.join("view_drc.sh");
    let macro_path = params.work_dir.join("dr.mac");

    let context = DrcRunsetContext {
        work_dir: params.work_dir,
        layout_path: params.layout_path,
        cell_name: params.cell_name,
        results_path: &results_path,
        summary_path: &summary_path,
    };
    let context = Context::from_serialize(context).map_err(Error::Tera)?;

    let contents = TEMPLATES
        .render("pegasusdrcctl", &context)
        .map_err(Error::Tera)?;

    fs::write(&runset_path, contents).map_err(Error::Io)?;

    let mut context = Context::new();
    context.insert("layout_path", params.layout_path);
    context.insert("macro_path", &macro_path);

    let contents = TEMPLATES
        .render("view_drc_lvs.sh", &context)
        .map_err(Error::Tera)?;

    fs::write(&view_drc_path, contents).map_err(Error::Io)?;
    let mut perms = fs::metadata(&view_drc_path)
        .map_err(Error::Io)?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&view_drc_path, perms).map_err(Error::Io)?;

    let mut context = Context::new();
    context.insert("work_dir", params.work_dir);

    let contents = TEMPLATES.render("dr.mac", &context).map_err(Error::Tera)?;

    fs::write(&macro_path, contents).map_err(Error::Io)?;

    Ok(DrcGeneratedPaths {
        runset_path,
        summary_path,
    })
}

pub fn write_drc_run_script(
    work_dir: impl AsRef<Path>,
    runset_path: impl AsRef<Path>,
    rules_path: impl AsRef<Path>,
) -> Result<PathBuf, Error> {
    fs::create_dir_all(&work_dir).map_err(Error::Io)?;

    let run_script_path = work_dir.as_ref().join("run_drc.sh");

    let mut context = Context::new();
    context.insert("runset_path", runset_path.as_ref());
    context.insert("rules_path", rules_path.as_ref());

    let contents = TEMPLATES
        .render("run_drc.sh", &context)
        .map_err(Error::Tera)?;

    fs::write(&run_script_path, contents).map_err(Error::Io)?;

    let mut perms = fs::metadata(&run_script_path)
        .map_err(Error::Io)?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&run_script_path, perms).map_err(Error::Io)?;

    Ok(run_script_path)
}

pub fn run_pegasus_drc(
    work_dir: impl AsRef<Path>,
    run_script_path: impl AsRef<Path>,
) -> Result<(), Error> {
    fs::create_dir_all(&work_dir).map_err(Error::Io)?;

    execute_run_script(run_script_path.as_ref(), &work_dir, "drc")
}

pub fn parse_pegasus_drc_results(rpt_path: impl AsRef<Path>) -> Result<DrcData, Error> {
    let re = Regex::new(r"^RULECHECK (.+) \.* TOTAL Result Count = (\d+)").unwrap();
    let file = fs::File::open(&rpt_path).map_err(Error::Io)?;
    let rule_checks: Vec<RuleCheck> = io::BufReader::new(file)
        .lines()
        .filter_map(|s| {
            if let Ok(line) = s {
                re.captures(&line).map(|caps| RuleCheck {
                    name: caps.get(1).unwrap().as_str().to_string(),
                    num_results: caps.get(2).unwrap().as_str().parse().unwrap(),
                })
            } else {
                None
            }
        })
        .filter(|rule_check| rule_check.num_results > 0)
        .collect();
    Ok(DrcData { rule_checks })
}

pub fn run_drc(params: &DrcParams) -> Result<DrcData, Error> {
    let DrcGeneratedPaths {
        runset_path,
        summary_path,
        ..
    } = write_drc_files(params)?;
    let run_script_path = write_drc_run_script(params.work_dir, runset_path, params.rules_path)?;
    run_pegasus_drc(params.work_dir, run_script_path)?;
    parse_pegasus_drc_results(summary_path)
}

#[cfg(test)]
mod tests {
    use crate::drc::{parse_pegasus_drc_results, run_drc, write_drc_files, DrcParams};
    use crate::tests::{EXAMPLES_PATH, SKY130_DRC_RULES_PATH, TEST_BUILD_PATH};
    use crate::RuleCheck;
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn test_write_drc_run_file() -> anyhow::Result<()> {
        let layout_path = PathBuf::from(EXAMPLES_PATH).join("gds/test_sky130_and3.gds");
        let work_dir = PathBuf::from(TEST_BUILD_PATH).join("test_write_drc_run_file");

        write_drc_files(&DrcParams {
            work_dir: &work_dir,
            layout_path: &layout_path,
            cell_name: "sky130_and3",
            rules_path: &PathBuf::from(SKY130_DRC_RULES_PATH),
        })?;
        Ok(())
    }

    #[test]
    fn test_parse_pegasus_drc_results() -> anyhow::Result<()> {
        let rpt_path = PathBuf::from(EXAMPLES_PATH).join("drc/drc.summary");

        let test_rules = HashMap::from([
            (
                "v_0_q0_mcon_NOTAreaidStdCellCore_added_vias".to_string(),
                5940,
            ),
            (
                "v_1_q0_mcon_NOTAreaidStdCellCore_added_below".to_string(),
                5940,
            ),
            (
                "v_2_q0_mcon_NOTAreaidStdCellCore_added_above".to_string(),
                5940,
            ),
            ("s_0_X.18".to_string(), 2156),
            (
                "v_3_q0_via_NOTAreaidStdCellCore_added_vias".to_string(),
                3772,
            ),
            (
                "v_4_q0_via_NOTAreaidStdCellCore_added_below".to_string(),
                3772,
            ),
            (
                "v_5_q0_via_NOTAreaidStdCellCore_added_above".to_string(),
                3772,
            ),
            ("s_1_X.18".to_string(), 1372),
            (
                "v_6_q0_via2_NOTAreaidStdCellCore_added_vias".to_string(),
                8540,
            ),
            (
                "v_7_q0_via2_NOTAreaidStdCellCore_added_below".to_string(),
                8540,
            ),
            (
                "v_8_q0_via2_NOTAreaidStdCellCore_added_above".to_string(),
                8540,
            ),
            ("s_2_X.18".to_string(), 4209),
        ]);

        let data = parse_pegasus_drc_results(rpt_path)?;

        for rule_check in data.rule_checks {
            if let Some(expected_num_results) = test_rules.get(&rule_check.name) {
                assert_eq!(
                    *expected_num_results,
                    rule_check.num_results,
                    "Incorrectly parsed DRC report, expected {} results for rule check {} but found {}",
                    expected_num_results,
                    &rule_check.name,
                    rule_check.num_results
                );
            }
        }

        Ok(())
    }

    fn test_check_filter(check: &RuleCheck) -> bool {
        check.name.starts_with("r_") && check.name != "r_1252_metblk.6"
    }

    #[test]
    fn test_run_drc() -> anyhow::Result<()> {
        let layout_path = PathBuf::from(EXAMPLES_PATH).join("gds/test_col_peripherals.gds");
        let work_dir = PathBuf::from(TEST_BUILD_PATH).join("test_run_drc");

        let data = run_drc(&DrcParams {
            work_dir: &work_dir,
            layout_path: &layout_path,
            cell_name: "col_peripherals",
            rules_path: &PathBuf::from(SKY130_DRC_RULES_PATH),
        })?;

        assert_eq!(
            data.rule_checks
                .into_iter()
                .filter(test_check_filter)
                .count(),
            0
        );

        Ok(())
    }

    #[test]
    fn test_run_drc_with_runset() -> anyhow::Result<()> {
        let layout_path = PathBuf::from(EXAMPLES_PATH).join("gds/test_col_peripherals.gds");
        let sram_runset_path = PathBuf::from(EXAMPLES_PATH).join("drc/sram_drc_runset");
        let work_dir = PathBuf::from(TEST_BUILD_PATH).join("test_run_drc_with_runset");

        let data = run_drc(&DrcParams {
            work_dir: &work_dir,
            layout_path: &layout_path,
            cell_name: "col_peripherals",
            rules_path: &PathBuf::from(SKY130_DRC_RULES_PATH),
        })?;

        assert_eq!(data.rule_checks.len(), 0);

        Ok(())
    }

    #[test]
    fn test_run_drc_fail() -> anyhow::Result<()> {
        let layout_path = PathBuf::from(EXAMPLES_PATH).join("gds/sram_sp_cell.gds");
        let work_dir = PathBuf::from(TEST_BUILD_PATH).join("test_run_drc_fail");

        assert_ne!(
            run_drc(&DrcParams {
                work_dir: &work_dir,
                layout_path: &layout_path,
                cell_name: "sky130_fd_bd_sram__sram_sp_cell",
                rules_path: &PathBuf::from(SKY130_DRC_RULES_PATH),
            })?
            .rule_checks
            .into_iter()
            .filter(test_check_filter)
            .count(),
            0
        );
        Ok(())
    }
}
