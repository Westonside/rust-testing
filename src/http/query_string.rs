use std::collections::HashMap;

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

//to represent the variants in the request where there is 0 args and multiple use an enum
pub enum Value<'buf> {
    Single(&'buf str),
    //use heap allocated array known as a Vector
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    //takes a reference to the query string and the key
    pub fn get(&self, key: &str) -> Option<&Value>{
        self.data.get(key)
    }
}
//implementing the from trait because this one cannot fail
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        //create an empty hashmap
        let mut data = HashMap::new();
        //for iterator returned by the split
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1 ..];
            }
            //if multiple type then push onto the vector
            //if key does not exist create a new value
            //if key does not exist then you insert else
            data.entry(key)
                //and modify takes a closure/anonymous function as param passes value from hash map
                .and_modify(|existing: &mut Value| match existing {
                    //for the single case create vector
                    Value::Single(prev_val) => {
                        //macro to create a new vector with values
                        //change multiple valued key
                        //have to dereference to follow the pointer and write the value of the new one
                        *existing = Value::Multiple(vec![prev_val,val]);
                    },
                    //if wrapping vector add value
                    Value::Multiple(vec) => {vec.push(val)}
                })
                .or_insert(Value::Single(val));
        }
        QueryString {data}
        unimplemented!();
    }
}