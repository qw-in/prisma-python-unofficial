use datamodel;
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use std::collections::{HashMap, BTreeMap};

#[pyclass]
#[derive(Clone)]
struct EngineOptions {
    datamodel: String,
    datasource_overrides: BTreeMap<String, String>,
    env: HashMap<String, String>,
    ignore_env_var_errors: bool,
}

#[pymethods]
impl EngineOptions {
    #[new]
    fn new(
        datamodel: String,
        datasource_overrides: BTreeMap<String, String>,
        env: HashMap<String, String>,
        ignore_env_var_errors: bool,
    ) -> Self {
        Self {
            datamodel,
            datasource_overrides,
            env,
            ignore_env_var_errors,
        }
    }
}

#[pyclass]
struct Engine {}

#[pymethods]
impl Engine {
    #[new]
    fn new(opts: EngineOptions) -> PyResult<Self> {
        let EngineOptions {
            datamodel,
			datasource_overrides,
            env,
            ignore_env_var_errors,
        } = opts;

		let overrides: Vec<(_, _)> = datasource_overrides.into_iter().collect();

        let config = if ignore_env_var_errors {
            datamodel::parse_configuration(&datamodel)
                .map_err(|err| PyOSError::new_err(err.to_string()))?
        } else {
            datamodel::parse_configuration(&datamodel)
                .and_then(|mut config| {
                    config
                        .subject
                        .resolve_datasource_urls_from_env(&overrides, |key| {
                            env.get(key).map(ToString::to_string)
                        })?;

                    Ok(config)
                })
                .map_err(|err| PyOSError::new_err(err.to_string()))?
        };

        println!("{:?}", env);

        Ok(Engine {})
    }

    fn connect<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        pyo3_asyncio::tokio::future_into_py(py, async {
			tokio::time::sleep(std::time::Duration::from_secs(1)).await;
			Ok(Python::with_gil(|py| py.None()))
		})
    }
}

#[pymodule]
fn query_engine(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;

    module.add_class::<EngineOptions>()?;
    module.add_class::<Engine>()?;

    Ok(())
}
