#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![deny(warnings)]

use pyo3::prelude::*;

mod diffusion;
mod domain1d;
pub mod prelude;

pub use domain1d::ImmersedNodeDomain1D;

#[pymodule(name = "domain1d")]
pub mod domain1d_py {
    #[pymodule_export]
    use super::domain1d::ImmersedNodeDomain1D;
}

#[pymodule(name = "diffusion")]
pub mod diffusion_py {
    #[pymodule_export]
    use super::diffusion::slycke::slycke_py;

    #[pymodule_export]
    use super::domain1d::ImmersedNodeDomain1D;

    #[pymodule_export]
    use super::diffusion::interstitial::CarbonitridingSolverPy as CarbonitridingSolver;

    #[pymodule_export]
    use super::diffusion::interstitial::ElementResults;
}

#[pymodule(name = "equations")]
pub mod equations {
    #[pymodule_export]
    use super::domain1d_py;
}
