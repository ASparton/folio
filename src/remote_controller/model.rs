use std::fmt::Display;

use serde::Deserialize;

/// GitHub organization fields
#[derive(Deserialize)]
pub struct GithubOrganization {
    login: String,
}

impl GithubOrganization {
    /// Determines wether the organization is considered as a portfolio or not.
    /// An organization is considered as a portfolio when its lowercase login ends with "portfolio".
    /// 
    /// # Examples
    /// 
    /// ```
    /// let org1 = GithubOrganization { login: "An organization" };
    /// assert_eq!(false, org1.is_portfolio());
    /// 
    /// let org2 = GithubOrganization { login: "MyPortfolio" };
    /// assert!(org2.is_portfolio());
    /// ```
    pub fn is_portfolio(&self) -> bool {
        self.login.to_lowercase().ends_with("portfolio")
    }
}

/// Describes a remote organization used to store portfolio projects.
#[derive(Debug)]
pub struct RemotePortfolio {
    name: String,
}

impl From<GithubOrganization> for RemotePortfolio {
    fn from(value: GithubOrganization) -> Self {
        RemotePortfolio { name: value.login }
    }
}

impl Display for RemotePortfolio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}