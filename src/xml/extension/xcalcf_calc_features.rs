use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct XcalcfCalcFeatures {
    #[serde(rename(serialize = "xcalcf:feature", deserialize = "feature"))]
    features: Vec<XcalcfFeature>,
}

impl Default for XcalcfCalcFeatures {
    fn default() -> Self {
        Self {
            features: create_default_xcalcf_calc_features(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct XcalcfFeature {
    #[serde(rename = "@name")]
    name: String,
}

impl XcalcfFeature {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

fn create_default_xcalcf_calc_features() -> Vec<XcalcfFeature> {
    let features: Vec<XcalcfFeature> = [
        "microsoft.com:RD",
        "microsoft.com:Single",
        "microsoft.com:FV",
        "microsoft.com:CNMTM",
        "microsoft.com:LET_WF",
        "microsoft.com:LAMBDA_WF",
        "microsoft.com:ARRAYTEXT_WF",
    ]
    .iter()
    .map(|name| XcalcfFeature::new(name.to_owned()))
    .collect();
    features
}
