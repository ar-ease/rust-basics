pub fn userfunc(){

    #[derive(Debug)]
#[allow(dead_code)]

   struct User {
    active : Option<bool>,
    username:String,
    email:Option<String>,
    sign_in_count: Option<u64>,
    }
       let user1 = User {
           active: Some(true),
           username : String::from("myusername"),
           email: Some(String::from("arease@gmail.com")),
           sign_in_count: Some(1),
       };

    let x: String = String::from("hii there ");
    println!("{}", x);
    println!("User 1 username is : {}" , user1.username);

    println!("{:#?}", user1);





}
