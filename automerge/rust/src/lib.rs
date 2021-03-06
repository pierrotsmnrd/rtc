use pyo3::prelude::*;
use std::fs::File;
// Logging
use log::LevelFilter;
use simplelog::*;
mod automerge_map;
mod nbformatbackend;
mod textarea;

// The main python module - jupyter_rtc_automerge
#[pymodule]
fn jupyter_rtc_automerge(py: Python, module: &PyModule) -> PyResult<()> {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("./jupyter_rtc_automerge.log").unwrap(),
        ),
    ])
    .unwrap();

    let submod_textarea = PyModule::new(py, "textarea")?;
    textarea::init_submodule(submod_textarea)?;
    module.add_submodule(submod_textarea)?;

    let submod_nbformatbackend = PyModule::new(py, "nb")?;
    nbformatbackend::init_submodule(submod_nbformatbackend)?;
    module.add_submodule(submod_nbformatbackend)?;

    let submod_ammap = PyModule::new(py, "automerge_map")?;
    automerge_map::init_submodule(submod_ammap)?;
    module.add_submodule(submod_ammap)?;

    Ok(())
}
