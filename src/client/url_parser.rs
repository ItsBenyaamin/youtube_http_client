use crate::app::error::Error;


#[derive(Debug, PartialEq)]
pub struct UrlParser {
    pub scheme: String,
    pub host: String,
    pub path: String
}

impl UrlParser {

    pub fn from(url: &str) -> Result<UrlParser, Error> {
        let addr = if url.starts_with("http") || url.starts_with("https") {
            url.to_owned()
        }else {
            format!("http://{}", url)
        };

        let mut split = addr.split("://");
        let scheme = split.next().unwrap().to_string();
        split = split.next().unwrap().split("/");

        let host = split.next().unwrap().to_string();

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
            UrlParser { scheme, host, path }
        )
    }

}

#[cfg(test)]
mod test {
    use super::UrlParser;

    #[test]
    fn test1_works() {
        let url = "https://benyaamin.com";
        let result = UrlParser::from(url).unwrap();

        let expected = UrlParser {
            scheme: "https".to_owned(),
            host: "benyaamin.com".to_owned(),
            path: "/".to_owned()
        };

        assert_eq!(result, expected)
    }

    #[test]
    fn test2_works() {
        let url = "benyaamin.com";
        let result = UrlParser::from(url).unwrap();

        let expected = UrlParser {
            scheme: "http".to_owned(),
            host: "benyaamin.com".to_owned(),
            path: "/".to_owned()
        };

        assert_eq!(result, expected)
    }

    #[test]
    fn test3_not_works() {
        let url = "benyaamin.com";
        let result = UrlParser::from(url).unwrap();

        let expected = UrlParser {
            scheme: "".to_owned(),
            host: "benyaamin.com".to_owned(),
            path: "/".to_owned()
        };

        assert_ne!(result, expected)
    }

}