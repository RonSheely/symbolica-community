use ::symbolica::api::python::{SymbolicaCommunityModule, create_symbolica_module};
use pyo3::{
    Bound, PyResult, pymodule,
    types::{PyAnyMethods, PyModule, PyModuleMethods},
};

#[cfg(feature = "python_stubgen")]
use pyo3_stub_gen::define_stub_info_gatherer;

fn register_extension<T: SymbolicaCommunityModule>(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(m.py(), &T::get_name())?;
    T::register_module(&child_module)?;
    m.add_submodule(&child_module)?;

    m.py().import("sys")?.getattr("modules")?.set_item(
        format!("symbolica.community.{}", T::get_name()),
        &child_module,
    )?;
    Ok(())
}

#[pymodule]
fn core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    create_symbolica_module(m)?;

    register_extension::<example::CommunityModule>(m)?;

    Ok(())
}

#[cfg(feature = "python_stubgen")]
define_stub_info_gatherer!(stub_info);
