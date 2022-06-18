use super::method::{MethodError,Method};
use super::{QueryString, QueryStringValue};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;


//LIFETIMES
//for lifetimes rust can tell the lifetime of a return type based on the first parameter if you are returning references to the first paramter becayse it implicityy gives a lifetime of 'a
//if have two params and are returning references you have to explicitly tell which one has the lifetime of what paramter because rust cannot infer which lifetime to give each reference to prevent dangling pointers
//lifetimes allow the developer to tell the compiler how long variables should last as to prevent dangling references

//example LIFETIME
//example function where an option is returned where the first string splice references a and b references b
//fn ex<'a, 'b>(request: &'a str, b: &'b str) -> Option<(&'a str, &'b str)>
//return Some((&request[i..], &request[i + 1..])) this has a type of Option(&'a str, &'a str)

//in most languages you do not have to worry about something being deallocated while you are using it because there are references to it in the code
//rust makes you think about the lifetime makes you make sure that references do not outlive array


//to use lifetimes in the struct have to make generic but will not be generic over a type but rather a lifetime represnt the lifetime with 'buf but normally call 'a
pub struct Request<'buf> {
    //need to do a lifetime specifier because the buffer could be deallocated and then the string slice would point to nothing
    //prevents dangling references
    //so gives guarentees like garbage collected languages give use the specifiers
    path: &'buf str,
    //now we can have an optional query string
    //option allows for us to have no value or some value
    query_string: Option<QueryString<'buf>>,
    //use option to be none or some(T)
    //super means go a level up to the parent
    method: Method,
}


//trait functionality type conversion
impl<'buf> TryFrom<&[u8]> for Request<'buf> {

    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {


        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)){
        //     Ok(request) =>{},
        //     Err(e) => return Err(e),
        // }
        //this is the equivalent of the above statement but better will convert the error but you need to convert the error from the utf 8 to parse error
        //another way is to do this which tells what to return if there is an error
        //let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        //tries to convert utf 8 error to parse error
        let request = str::from_utf8(buf)?;
        //now have a string splice so need to break the string up

        // match get_next_word(request) {
        //     Some((method, request)) =>{},
        //     None => return Err(ParseError::InvalidRequest)
        // }
        //can do ok or which transforms option to a result maps to an error that is passed to the function
        let (method,request) =  get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        //to call again
        let (mut path,request) =  get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) =  get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        //because parse returns a result but now need to implement from for method error since the parse function returns a method error
        let method: Method  = method.parse()?;
        let mut query_string = None;

        //make query string that comes after the question mark in the request
        // match path.find('?'){//returns an option
        //     Some(i) =>{
        //         //everything after  question mark forward
        //         query_string = Some(&path[i + 1..]);
        //         //everything before
        //         path = &path[..1];
        //     }
        //     None =>{}
        // }

        // let q = path.find('?');
        // if q.is_some(){
        //     //breaks if option is none
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     //         //everything before
        //     path = &path[..1];
        // }

        //to avoid defining a new variable like above use if let
        //if returns some then do if statement unwraps contents into the variable i
        if let Some(i) = path.find('?'){
            query_string = Some(QueryString::from(&path[i + 1..]));
                //         //everything before
                path = &path[..1];
        }


        //reusing var names and overwrite is called variable shadowing
        //returns a result
        Ok(Self {
            path: path,
            query_string,
            method
        })
    }
}
//returns a tuple contains the current word and then does the next word can be none
fn get_next_word(request: &str) -> Option<(&str, &str)>{
    //loops through all of the elements using an iterator enumerate provides the current index returns (index, value)
    for (i,c) in request.chars().enumerate(){
        if c == ' ' || c == '\r'{
            //return all characters before the space and after the space
            //can do +1 because we know that a space is only one byte long
            return Some((&request[..i], &request[i+1..]))
        }
    }
    None
}


trait Encrypt {
    fn encrypt(&self) -> Self;
}
//allows the encryption for a string in this case
impl Encrypt for String {
    fn encrypt(&self) -> Self {
        unimplemented!();
    }
}
//extended the functionality for a byte array
impl Encrypt for &[u8] {
    fn encrypt(&self) -> Self{
        unimplemented!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}



impl ParseError{
    fn message(&self) -> &str{
        match self {
            Self::InvalidRequest=> "Invalid Response",
            Self::InvalidEncoding=> "Invalid Encoding",
            Self::InvalidProtocol=> "Invalid Protocol",
            Self::InvalidMethod=> "Invalid Method",


        }
    }
}


//this satisfies the question mark because now if fails can return a method error
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

//allows to convert the utf 8 error to the parse error
impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}


impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        //generate and write string to formatter
        write!(f,"{}", self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        //generate and write string to formatter
        write!(f,"{}", self.message())
    }
}

//"{:?} is the debug formatter
//implement error trait forces us to meet certain points need to implement display adn debug
impl Error for ParseError{

}

