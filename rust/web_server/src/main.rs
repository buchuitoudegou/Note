mod req_dealer;
use req_dealer::router::router::Router;
use req_dealer::context::Context;
use req_dealer::req_dealer::ReqDealer;

fn main() {
    let mut tcp_server = ReqDealer::new();
    let mut router = Router::new();
    router.register("/user", Box::new(|ctx: &mut Context| {
        println!("user {}", ctx.req.method);
        ctx.resp.status = 400;
        ctx.resp.body = String::from("hello user\n");
    })).register("/login", Box::new(|ctx: &mut Context| {
        println!("login {}", ctx.req.method);
        ctx.resp.body = String::from("hello login\n");
        ctx.resp.status = 300;
    }));
    tcp_server.register(Box::new(router));
    let server = tcp_server.at("127.0.0.1:7878");
    if let Ok(_) = server {
        tcp_server.listen();
    } else if let Err(err) = server {
        println!("{}", err);
    } else {
        panic!("unknown error");
    }
}