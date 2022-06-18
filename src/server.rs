use std::io::Read;
use std::net::TcpListener;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Server {
        address: String,
    }
    //use pointer because do not need to know the size this is a slice
    fn arr (a: &[u8]){

    }

    //implementation block for the server struct, adds the functionality
    impl Server {
        //method are defined in the context of a struct methods always take self as the first arg
        //associated functions are functions that are associated with the struct type but do not need an instance like a static function
        //use :: to access the associated function
        //Self and Server interchangeable in this case
        pub fn new(address: String) -> Self {
            Server {
                //if the names are the same
                address
            }
        }
        //run function takes ownership of the whole entire struct
        //struct deallocated when run is done do because runs forever
        pub fn run(self) {
            println!("listening on {}", self.address);
            //returns a result enum
            //result for error handling ensures handle errors
            //two types of errors: recoverable (file not found) and unrecoverable cannot be handled because of a bug (index out of bounds)
            ///error or ok
            ///this can have a recoverable error
            /// returns value if ok else will terminate the program and log the error
            let listener = TcpListener::bind(&self.address).unwrap();
            //same as while true added annotaions for the lop allows for us to break out of certain loops
            'outer: loop{
                //match works on strings too like a switch statement
                //can  do a || b => {}
                match listener.accept(){
                    //do not care about address
                    Ok((mut stream,_)) => {
                        //buffer that is 0 initalized with 1024 bytes rust initalizes so that avoid corrupted memory
                        let mut buffer = [0; 1024];
                        // arr(&a[1..3]);
                        match stream.read(&mut buffer) {
                            Ok(_) =>{
                                println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                                //have to pull the trait into the scope before can use
                                //have to convert the buffer so that compiler knows the type since try from is generic
                                //tell compiler to read as a slice
                                match Request::try_from(&buffer[..]){
                                    //what we know is that the return (request) has a relationship with what is in the buffer
                                    Ok(request) =>{
                                        //the compiler will let you modify the buffer if it sees that you are not modifying the request because it knows that request internally borrows buffer
                                    },
                                    Err(e) =>println!("failure to parse a request: {}", e),
                                }


                                //cannot infer which type so you have to tell which type you want same thing
                                // let res: &Result<Request,_> = &buffer[..].try_into();

                            },
                            Err(e) =>println!("failure {}", e.to_string()),
                        }
                    },
                    //can do _ => to match everything else default case
                    Err(e) => println!("Failure! {}",e.to_string()),
                }


            }

            let tup = (1,2);
        }
    }
