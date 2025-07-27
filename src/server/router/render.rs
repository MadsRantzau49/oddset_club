use tera::{Tera, Context};

use lazy_static::lazy_static;

use crate::server::router::dashboard::load_dashboard_context;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        Tera::new("src/frontend/html/**/*").expect("Failed to load templates")
    };
}

pub fn get_html(name: &str, club_id: i64) -> String{
    let mut context = Context::new();
    if club_id > 0{
        context = load_dashboard_context(club_id);
        return render_template("dashboard.html", &context);
    }
    return match TEMPLATES.render(&name, &context) {
        Ok(rendered) => format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", rendered),
        Err(_) => "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nTemplate rendering error".to_string(),
    }
}

pub fn render_template(name: &str, context: &Context) -> String{
    return match TEMPLATES.render(&name, &context) {
        Ok(rendered) => format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", rendered),
        Err(_) => "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nTemplate rendering error".to_string(),
    }
}

pub fn render_error(error_msg: &str) -> String {
    let mut context = Context::new();
    context.insert("message", &error_msg);
    render_template("error.html", &context)
}

pub fn render_set_session_cookie(name: &str, context: &Context, session_id: String) -> String {
    return match TEMPLATES.render(&name, &context) {
        Ok(rendered) => 
        format!(
            "HTTP/1.1 200 OK\r\nSet-Cookie: session_id={}; Path=/; HttpOnly\r\nContent-Length: {}\r\n\r\n{}", 
            session_id, rendered.len(), rendered
        ),
        Err(_) => "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nTemplate rendering error".to_string(),
    }
}