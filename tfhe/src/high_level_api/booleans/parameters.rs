use crate::boolean::parameters::{
    BooleanParameters, DecompositionBaseLog, DecompositionLevelCount, GlweDimension, LweDimension,
    PolynomialSize, StandardDev,
};
pub use crate::boolean::parameters::{DEFAULT_PARAMETERS, TFHE_LIB_PARAMETERS};

use serde::{Deserialize, Serialize};

pub trait BooleanParameterSet: Into<BooleanParameters> {
    type Id: Copy;
}

/// Parameters for [FheBool].
///
/// [FheBool]: crate::high_level_api::FheBool
#[cfg_attr(all(doc, not(doctest)), cfg(feature = "boolean"))]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FheBoolParameters {
    pub lwe_dimension: LweDimension,
    pub glwe_dimension: GlweDimension,
    pub polynomial_size: PolynomialSize,
    pub lwe_modular_std_dev: StandardDev,
    pub glwe_modular_std_dev: StandardDev,
    pub pbs_base_log: DecompositionBaseLog,
    pub pbs_level: DecompositionLevelCount,
    pub ks_base_log: DecompositionBaseLog,
    pub ks_level: DecompositionLevelCount,
}

impl FheBoolParameters {
    pub fn tfhe_lib() -> Self {
        Self::from_static(&TFHE_LIB_PARAMETERS)
    }

    fn from_static(params: &'static BooleanParameters) -> Self {
        (*params).into()
    }
}

impl Default for FheBoolParameters {
    fn default() -> Self {
        Self::from_static(&DEFAULT_PARAMETERS)
    }
}

impl From<FheBoolParameters> for BooleanParameters {
    fn from(params: FheBoolParameters) -> Self {
        Self {
            lwe_dimension: params.lwe_dimension,
            glwe_dimension: params.glwe_dimension,
            polynomial_size: params.polynomial_size,
            lwe_modular_std_dev: params.lwe_modular_std_dev,
            glwe_modular_std_dev: params.glwe_modular_std_dev,
            pbs_base_log: params.pbs_base_log,
            pbs_level: params.pbs_level,
            ks_base_log: params.ks_base_log,
            ks_level: params.ks_level,
        }
    }
}

impl From<BooleanParameters> for FheBoolParameters {
    fn from(params: BooleanParameters) -> FheBoolParameters {
        Self {
            lwe_dimension: params.lwe_dimension,
            glwe_dimension: params.glwe_dimension,
            polynomial_size: params.polynomial_size,
            lwe_modular_std_dev: params.lwe_modular_std_dev,
            glwe_modular_std_dev: params.glwe_modular_std_dev,
            pbs_base_log: params.pbs_base_log,
            pbs_level: params.pbs_level,
            ks_base_log: params.ks_base_log,
            ks_level: params.ks_level,
        }
    }
}
