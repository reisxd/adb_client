use pyo3_stub_gen::{Result, StubInfo};

fn main() -> Result<()> {
    // let stub = pyadb_client::stub_info().expect("eee");
    let stub = StubInfo::from_pyproject_toml("pyproject.toml").expect("eee");
    dbg!(&stub);
    stub.generate().expect("cannot generate");
    Ok(())
}
