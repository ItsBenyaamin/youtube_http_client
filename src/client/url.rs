use crate::app::error::Error;


#[derive(Debug, PartialEq)]
pub struct ParsedUrl {
    pub scheme: String,
    pub host: String,
    pub path: String
}

impl ParsedUrl {

    pub fn from(url: &str) -> Result<ParsedUrl, Error> {
        let addr = if url.starts_with("http") || url.starts_with("https") {
            url.to_owned()
        }else {
            format!("http://{}", url)
        };

        let mut split = addr.split("://");

        let scheme = match split.next() {
            Some(v) => v.to_string(),
            None => return Err(Error::UrlParsingError),
        };

        split = match split.next() {
            Some(v) => v.split("/"),
            None => return Err(Error::UrlParsingError),
        };

        let host = match split.next() {
            Some(v) => v.to_string(),
            None => return Err(Error::UrlParsingError),
        };

        let mut path = String::new();
        loop {
            match split.next() {
                Some(v) => path.push_str(
                    format!("/{}", v).as_str()
                ),
                None => {
                    if path.is_empty() {
                        path.push('/');
                    }
                    break;
                },
            }
        }


        Ok(
            ParsedUrl { scheme, host, path }
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
            path: "/".to_owned()
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
            path: "/".to_owned()
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
            path: "/".to_owned()
        };

        assert_ne!(result, expected)
    }

    #[test]
    fn test4_works() {
        let url = "168.119.172.64";
        let result = ParsedUrl::from(url).unwrap();

        let expected = ParsedUrl {
            scheme: "".to_owned(),
            host: "168.119.172.64".to_owned(),
            path: "/".to_owned()
        };

        assert_ne!(result, expected)
    }

}