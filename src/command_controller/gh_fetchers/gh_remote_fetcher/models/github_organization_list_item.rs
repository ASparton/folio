use serde::Deserialize;

/// Returned when listing all the orgs of a user
#[derive(Deserialize)]
pub struct GithubOrganizationListItem {
    pub login: String,
}

impl GithubOrganizationListItem {
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