use bytes::Bytes;
use http;
use wasi_experimental_http::request;

#[no_mangle]
pub extern fn hello(name: u32){
    println!("{}",name);
    let url = "https://postman-echo.com/post".to_string();
    let req = http::request::Builder::new()
        .method(http::Method::POST)
        .uri(&url)
        .header("Content-Type", "text/plain")
        .header("abc", "def");
    let b = Bytes::from("Testing with a request body. Does this actually work?");
    let req = req.body(Some(b)).unwrap();

    let mut res = request(req).unwrap();
    let str = std::str::from_utf8(&res.body_read_all().unwrap()).unwrap().to_string();
    println!("{:#?}", res.header_get("Content-Type".to_string()));
    println!("{}", str);


    
    // println!("{:#?}", res.status_code);
}

