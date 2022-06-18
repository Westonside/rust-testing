use std::str::FromStr;



pub enum Method {
    //every variant will contain enough to hold the largest enum think union so it does not know the size everyone will hold GET size

    //get variant will contain a string
    GET,
    //user id
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method{
    type Err = String;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTION" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),

        }

        unimplemented!()
    }
}

pub struct MethodError;