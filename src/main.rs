use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new()
        .service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// fn run_app() -> std::io::Result<()>{
//     let sys = System::new();
//
//     let srv = HttpServer::new(||{
//         App::new()
//             .wrap(middleware::Logger::default())
//             .service(web::resource("/index.html").route(web::get()).to(greet))
//             .service(index)
//     }).bind("127.0.0.1:8080")?.run();
//     return sys.block_on(srv);
// }
//
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

// fn main() {
//     let a = 10;
//     let b = a;
//     let str1= String::from("peng");
//     let str2 = str1;
//     println!("{}",str2);
//     //println!(a);
//     println!("Hello, world!");
//     let stu = Student{
//         id:1,
//         name:String::from("pengshuai"),
//         age:30
//     };
//     println!("stu: {:?}",stu);
//
//
//     let bookType = Book::Electronic;
//     println!("{:?}",bookType);
//
//     match bookType {
//         Book::Papery => {println!("the book is Papery")}
//         Book::Electronic => {println!("the book is Electronic")}
//     }
//     test1();
// }
//
// #[derive(Debug)]
// struct Student{
//     id:i32,
//     name:String,
//     age:i32,
// }
// #[derive(Debug)]
// enum Book{
//     Papery,Electronic
// }
//
// fn longer<'a>(s1:&'a str,s2:&'a str)->&'a str{
//     if s1.len()<s2.len() {
//         s2
//     }else{
//         s1
//     }
// }
//
// fn test1(){
//     let r;
//     {
//         let s1 ="rust";
//         let s2 = "pengshuai";
//         r = longer(s1,s2);
//     }
//     println!("{}",r);
// }
