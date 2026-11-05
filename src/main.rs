enum Status {
    SUCCESS,
    PENDING,
    UNREAD
}

fn test_data(data : (String,Status)){
    match data.1 {
        Status::PENDING => ouput_data("Pending"),
        Status::SUCCESS => ouput_data("Success"),
        _ => ouput_data("Unread"),
    };

}

fn ouput_data(data : &str){
    println!("Status : {}", data)
}

// async fn call_api() -> Result<serde_json::Value, reqwest::Error>{
//     let client = reqwest::Client::new();

//     let todo = client.get("https://jsonplaceholder.typicode.com/todos/1")
//         .send()
//         .await?
//         .error_for_status()?
//         .json()
//         .await?;

//     Ok(todo)
// }

// fn string_not_str(data : (String,&str,Box<str>)){
//     println!("{:?}",data)
// }

#[tokio::main]
async fn main() {
    let _ = test_data((
        "wafi ".to_string(),
        Status::UNREAD,
    ));


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
