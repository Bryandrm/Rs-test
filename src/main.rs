//---------------------------------------------clase 14---------------------------------------------//

//Enums = enumerations


struct User{
    name: String,
    email: String,
     activo: bool,
    user_role: UserRole,
    website: Website, 
}

enum UserRole{
    BASIC,
    ADMIN,
}

enum Website{
    URL(String),
    INSTAGRAM(String),
    LINKEDIN(String),
    FACEBOOK(String),
}

fn main() {
    let mut user = User{
        name: "Julio".to_string(),
        email: String::from("algo@algo.com"),
        activo: true,
        user_role: UserRole::BASIC,
        website: Website::INSTAGRAM(String::from("BDR")),
    };

    let access = hasAccess(user.user_role);
}

fn hasAccess(user_role: UserRole)-> bool{
    match  user_role{
        UserRole::ADMIN => true,
        UserRole::BASIC => false,

    }
}


fn goToWebsite(website: Website) {
    match  website{
        Website::INSTAGRAM => // has algo ...website
        Website::FACEBOOK => // .... 

    }
}















//---------------------------------------------clase 13---------------------------------------------//
/*
struct Usuario{
    nombre: String,
    email: String,
    edad: i32,
    activo: bool,
}

fn main() {
    let  mut user  = Usuario {
        nombre: "Julio".to_string(),
        email: String::from("algo@algo.com"),
        edad: 99,
        activo: true,
    };

    print!("usuario {}, edad {}", user.nombre, user.edad);

    user.activo = false;

    let user1 = nuevo_usuario( String::from("algo@algo.com"),  String::from("Julio"))
    let user2  = Usuario{
        nombre:"Juan".to_string(),
        email: "otroEmail@mail.com".to_string(),
        ..user1
    };
    
 }

 fn nuevo_usuario(nombre: String, email:String)-> Usuario {
     return Usuario{
         nombre,
         email,
         edad: 100,
         activo: true,
     }

 }
 */