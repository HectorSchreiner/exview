use std::{net::Ipv4Addr, str::FromStr};

pub struct DrillDown {
    options: DrillDownSearchOptions,
}

impl DrillDown {
    pub fn new(builder: DrillDownSearchOptions) -> Self {
        todo!()
    }
}

/// Drilldownsearchbuilder is used to create a drilldown.
pub struct DrillDownSearchOptions {
    user: Option<String>,
    ip_origin: Option<std::net::Ipv4Addr>,
    ip_impacted: Option<std::net::Ipv4Addr>,
    port_origin: Option<i32>,
    port_impacted: Option<i32>,
    log_source_entity: Option<String>,
    session: Option<i32>
}

impl DrillDownSearchOptions {
    pub fn new(builder: &DrillDownSearchOptionsBuilder) -> DrillDownSearchOptions {
        builder.build()
    }
}

pub struct DrillDownSearchOptionsBuilder {
    user: Option<String>,
    ip_origin: Option<std::net::Ipv4Addr>,
    ip_impacted: Option<std::net::Ipv4Addr>,
    port_origin: i32,
    port_impacted: i32,
    log_source_entity: String,
    session: i32
}

impl DrillDownSearchOptionsBuilder {
    pub fn user(&mut self, user: &str) -> &mut Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn ip_origin(&mut self, ip: &str) -> &mut Self {
        match Ipv4Addr::from_str(ip) {
            Ok(ip) => self.ip_origin = Some(ip),
            Err(e) => todo!()
        }
        self
    }

    pub fn build(&self) -> DrillDownSearchOptions {
        todo!()
    }
}