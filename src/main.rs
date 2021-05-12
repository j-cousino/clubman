#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;

use std::path::{Path, PathBuf};
use std::string::String;

use rocket::outcome::IntoOutcome;
use rocket::request::{self, Form, FlashMessage, FromRequest, Request};
use rocket::response::{NamedFile, Redirect, Flash};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::Template;

static LAYOUT: &str = "components/layout";

#[derive(Serialize)]
struct TemplateContext<T: ::serde::Serialize> {
    title: String,
    parent: &'static str,
    data: T,
    is_member: bool,
}

impl<T: ::serde::Serialize> TemplateContext<T> {
    fn new(title: String, data: T, is_member: bool) -> Self {
        Self {
            title,
            parent: LAYOUT,
            data,
            is_member,
        }
    }
}


// Starting up the app start on the index page.
// This is public and just a club advert.
#[get("/", rank = 2)]
fn guest_home() -> Template {
    Template::render("index", &TemplateContext::new( "Home".to_string(), (), false ))
}

#[get("/")]
fn member_home(member: Member) -> Template {
    Template::render("index", &TemplateContext::new("Home".to_string(), member.0, true ))
}

// This is needed for pages to access static elements
#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[derive(FromForm)]
struct SignIn {
    member_email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct Member(usize);

impl<'a, 'r> FromRequest<'a, 'r> for Member {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Member, !> {
        request.cookies()
            .get_private("member_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| Member(id))
            .or_forward(())
    }
}

// Accepts the sign in form
#[post("/signin", data = "<signin>")]
fn signin(mut cookies: Cookies, signin: Form<SignIn>) -> Result<Redirect, Flash<Redirect>> {
    if signin.member_email == "Sergio" && signin.password == "password" {
        cookies.add_private(Cookie::new("member_id", 1.to_string()));
        Ok(Redirect::to(uri!(member_home)))
    } else {
        Err(Flash::error(Redirect::to(uri!(signin_error)), "Invalid username/password."))
    }
}

#[get("/signin")]
fn signin_member(_member: Member) -> Template {
    Template::render("signin", &TemplateContext::new("Sign in".to_string(), (), false))
}

#[get("/signin", rank = 2)]
fn signin_error(flash: Option<FlashMessage>) -> Template {
    if let Some(ref msg) = flash {
        Template::render("signin", &TemplateContext::new("Sign in".to_string(), msg.msg(), false))
    }
    else{
        Template::render("signin", &TemplateContext::new("Sign in".to_string(), (), false))
    }

}

#[get("/signout")]
fn signout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("member_id"));
    Flash::success(Redirect::to(uri!(signin_error)), "Successfully logged out.")
}


#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn main() {
    rocket::ignite()
        .mount(
            "/", 
            routes![ 
                guest_home, 
                member_home, 
                files, 
                signin, 
                signin_member, 
                signin_error,
                signout
            ])
        .register(catchers![not_found])
        .attach(Template::fairing())
        .launch();
}

