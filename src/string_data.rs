pub fn test_string(){
      // readonly memory
    let data_string = "wafiuddin";
    // add to heap memory location, read only memory
    let boxed : Box<str> = data_string.into(); 
    println!("{}",boxed);
    
    // added to heap memory, butt can be add value to string
    let mut s = String::from(data_string);
    s.push_str("wafiq");
    
    // back to Box
    let boxed2 = s.into_boxed_str();
    println!("{}",boxed2.len());
    println!("{}",boxed2);

    // back to string
    let back_data : Box<str> = boxed2.into();
    println!("{}",back_data);
}