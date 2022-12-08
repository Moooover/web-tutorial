#[derive(Debug, PartialEq)]
pub enum Version {
    V0_9,
    V1_0,
    V1_1,
    V2,
    V3,
}

impl Version {
    pub fn from(string: &str) -> Result<Version, &str> {
        use Version::*;
        match string {
            "HTTP/0.9" => Ok(V0_9),
            "HTTP/1.0" => Ok(V1_0),
            "HTTP/1.1" => Ok(V1_1),
            "HTTP/2" => Ok(V2),
            "HTTP/3" => Ok(V3),
            _ => Err("Couldn't parse version from string"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retrieve_version_from_string() {
        assert_eq!(Version::V0_9, Version::from("HTTP/0.9").unwrap());
        assert_eq!(Version::V1_0, Version::from("HTTP/1.0").unwrap());
        assert_eq!(Version::V1_1, Version::from("HTTP/1.1").unwrap());
        assert_eq!(Version::V2, Version::from("HTTP/2").unwrap());
        assert_eq!(Version::V3, Version::from("HTTP/3").unwrap());
    }

    #[test]
    #[should_panic(expected = "parse version from string")]
    fn retrieve_version_from_string_fail() {
        Version::from("HTTP/0.8").unwrap();
    }
}
