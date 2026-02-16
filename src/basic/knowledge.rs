// // OWNERSHIP
// let s = String::from("hello"); // s owns the data
// let s2 = s;                    // s move ke s2, s not valid lagi
//
// // BORROWING (Reference)
// let s = String::from("hello");
// let r = &s;                    // r borrow s, s is valid
//
// // MUTABLE REFERENCE
// let mut s = String::from("hello");
// let r = &mut s;                // can modify through r
// r.push_str(" world");
//
// // OPTION
// let maybe = Some(5);
// let value = maybe.unwrap();           // get 5 or panic
// let value = maybe.unwrap_or(0);       // get 5,or  0 if None
// let value = maybe.expect("kosong!");  // panic with message custom
//
// // RESULT
// let result = Ok(10);
// let value = result?;                  // extract or return error
// let value = result.unwrap();          // extract or panic
// let value = result.unwrap_or(0);      // extract or default
//
// // CONVERSIONS
// let s = "hello";
// s.as_bytes()   // &str → &[u8]
// s.as_ref()     // convert ke reference
// s.to_string()  // &str → String (new allocate)
// s.to_owned()   // with to_string()