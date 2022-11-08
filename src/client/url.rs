use crate::app::error::Error;


#[derive(Debug, PartialEq)]
pub struct ParsedUrl {
    pub scheme: String,
    pub host: String,
    pub port: usize,
    pub path: String,
    pub file: Option<String>
}

impl ParsedUrl {

    pub fn from(url: &str) -> Result<ParsedUrl, Error> {
        let mut file: Option<String> = None;
        let mut port: usize = 0;

        let addr = if url.starts_with("http") || url.starts_with("https") {
            url.to_owned()
        }else {
            format!("http://{}", url)
        };

        let mut split = addr.split("://");

        let scheme = match split.next() {
            Some(v) => {
                if v == "https" {
                    port = 443;
                }else {
                    port = 80;
                }
                v.to_string()
            },
            None => return Err(Error::UrlParsingError),
        };

        split = match split.next() {
            Some(v) => v.split("/"),
            None => return Err(Error::UrlParsingError),
        };

        let host = match split.next() {
            Some(v) => {
                if v.contains(":") {
                    let mut host_split = v.split(":");
                    let host = host_split.next().unwrap();
                    port = host_split.next().unwrap().parse().unwrap();
                    host.to_string()
                }else {
                    v.to_string()
                }
            },
            None => return Err(Error::UrlParsingError),
        };

        let mut path = String::new();
        loop {
            match split.next() {
                Some(v) => {
                    path.push_str(format!("/{}", v).as_str());

                    if v.contains('.') {
                        file = Some(String::from(v));
                    }
                },
                None => {
                    if path.is_empty() {
                        path.push('/');
                    }
                    break;
                },
            }
        }


        Ok(
            ParsedUrl { scheme, host, port, path, file }
        )
    }

}

#[cfg(test)]
mod test {
    use super::ParsedUrl;

    #[test]
    fn test1_works() {
        let url = "https://benyaamin.com";
        let result = ParsedUrl::from(url).unwrap();

        let expected = ParsedUrl {
            scheme: "https".to_owned(),
            host: "benyaamin.com".to_owned(),
            port: 443,
            path: "/".to_owned(),
            file: None
        };

        assert_eq!(result, expected)
    }

    #[test]
    fn test2_works() {
        let url = "benyaamin.com";
        let result = ParsedUrl::from(url).unwrap();

        let expected = ParsedUrl {
            scheme: "http".to_owned(),
            host: "benyaamin.com".to_owned(),
            port: 80,
            path: "/".to_owned(),
            file: None
        };

        assert_eq!(result, expected)
    }

    #[test]
    fn test3_not_works() {
        let url = "benyaamin.com";
        let result = ParsedUrl::from(url).unwrap();

        let expected = ParsedUrl {
            scheme: "".to_owned(),
            host: "benyaamin.com".to_owned(),
            port: 80,
            path: "/".to_owned(),
            file: None
        };

        assert_ne!(result, expected)
    }

    #[test]
    fn test4_works() {
        let url = "168.119.172.64";
        let result = ParsedUrl::from(url).unwrap();

        let expected = ParsedUrl {
            scheme: "http".to_owned(),
            host: "168.119.172.64".to_owned(),
            port: 80,
            path: "/".to_owned(),
            file: None
        };

        assert_eq!(result, expected)
    }

    #[test]
    fn test5_works() {
        let url = "168.119.172.64:5481";
        let result = ParsedUrl::from(url).unwrap();

        let expected = ParsedUrl {
            scheme: "http".to_owned(),
            host: "168.119.172.64".to_owned(),
            port: 5481,
            path: "/".to_owned(),
            file: None
        };

        assert_eq!(result, expected)
    }

    #[test]
    fn test6_works() {
        let url = "168.119.172.64/Documents.rar";
        let result = ParsedUrl::from(url).unwrap();

        let expected = ParsedUrl {
            scheme: "http".to_owned(),
            host: "168.119.172.64".to_owned(),
            port: 80,
            path: "/Documents.rar".to_owned(),
            file: Some("Documents.rar".to_string())
        };

        assert_eq!(result, expected)
    }

}