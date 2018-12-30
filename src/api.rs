use crate::auth::AccessToken;
use reqwest;

#[derive(Debug)]
pub struct Pagination {
  /// An epoch timestamp to use for filtering results
  /// that have taken place before a certain time.
  /// (when using time-based cursors)
  pub before: u64,

  /// An epoch timestamp to use for filtering results
  /// that have taken place after a certain time.
  /// (when using time-based cursors)
  pub after: u64,

  /// Page number (when using page-based cursors)
  pub page: u32,

  /// Number of items per page. Defaults to 30
  pub per_page: u32,
}

impl Pagination {
  pub fn as_query(&self) -> Vec<(&'static str, u64)> {
    let mut query: Vec<(&'static str, u64)> = Vec::new();
    if self.before > 0 {
      query.push(("before", self.before));
    }
    if self.after > 0 {
      query.push(("after", self.after));
    }
    if self.page > 0 {
      query.push(("page", self.page as u64));
    }
    if self.per_page > 0 {
      query.push(("per_page", self.per_page as u64));
    }
    query
  }
}

impl Default for Pagination {
  fn default() -> Pagination {
    Pagination {
      before: 0,
      after: 0,
      page: 0,
      per_page: 30,
    }
  }
}

// --

#[derive(Debug)]
pub struct Context {
  pub access_token: AccessToken,
}

pub type ApiResult<T> = reqwest::Result<T>;

// --

#[derive(Debug)]
pub struct Api {
  client: reqwest::Client,
  base_url: &'static str,
}

impl Api {
  pub fn new(base_url: &'static str) -> Self {
    Api {
      client: reqwest::Client::new(),
      base_url,
    }
  }

  pub fn get(&self, path: &str, context: &Context) -> reqwest::Result<reqwest::Response> {
    let url = [self.base_url, path].join("");
    self
      .client
      .get(&url)
      .bearer_auth(&context.access_token)
      .send()?
      .error_for_status()
  }

  pub fn get_paginated(
    &self,
    path: &str,
    context: &Context,
    pagination: &Pagination,
  ) -> reqwest::Result<reqwest::Response> {
    let url = [self.base_url, path].join("");
    self
      .client
      .get(&url)
      .query(&pagination.as_query())
      .bearer_auth(&context.access_token)
      .send()?
      .error_for_status()
  }
}
