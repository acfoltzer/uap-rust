use yaml_rust::{Yaml};
use yaml;
use regex::Regex;


///`UserAgent` contains the user agent information.
#[derive(Debug, PartialEq, Eq)]
pub struct UserAgent {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

#[derive(Debug)]
pub struct UserAgentParser {
    pub regex: Regex,
    pub family: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

impl UserAgentParser {
    pub fn from_yaml(y: &Yaml) -> Option<UserAgentParser> {
            yaml::string_from_map(y, "regex")
            .map(|r| r.replace(r"\-", r"-"))
            .map(|r| r.replace(r"\ ", r" "))
            .map(|r| r.replace(r"\/", r"/"))
            .and_then(|r| Regex::new(&r[..]).ok())
            .map(|r| UserAgentParser {
                regex: r,
                family: yaml::string_from_map(y, "family_replacement"),
                major: yaml::string_from_map(y, "v1_replacement"),
                minor: yaml::string_from_map(y, "v2_replacement"),
                patch: yaml::string_from_map(y, "v3_replacement"),
            })
    }

    pub fn parse(&self, agent: String) -> Option<UserAgent> {
        self.regex.captures(&agent[..]).map(|c| {
            let family = self.family.clone()
                .and_then(|f| c.get(1).map(|a| f.replace("$1", a.as_str())))
                .unwrap_or(c.get(1).map(|m| m.as_str()).unwrap_or("Other").to_owned());
            let major = self.major.clone()
                .and_then(|f| c.get(2).map(|a| f.replace("$2", a.as_str())))
                .or(c.get(2).map(|m| m.as_str().to_owned()));
            let minor = self.minor.clone()
                .and_then(|f| c.get(3).map(|a| f.replace("$3", a.as_str())))
                .or(c.get(3).map(|m| m.as_str().to_owned()));
            let patch = self.patch.clone()
                .and_then(|f| c.get(4).map(|a| f.replace("$4", a.as_str())))
                .or(c.get(4).map(|m| m.as_str().to_owned()));

            UserAgent {
                family: family,
                major: major,
                minor: minor,
                patch: patch,
            }
        })
    }
}
