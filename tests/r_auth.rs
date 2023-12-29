#[cfg(test)]
mod tests {
    
    use actix_web::test;
    use actix_web::web::{Data,Json};
    use serde_json::json;
    use server::r_auth::register;
    use server::s_state::State;

    #[actix_rt::test]
    async fn test_register() {
        
        let req = test::TestRequest::post()
        .set_json(json!({
            "email": "abctest@gmail.com",
            "password": "ppp",
        }))
        .to_request();

        let state = State::default().await;
        // let resp = register(
        //     Data::new(state), 
        //     Json::from(req),
        // ).await;
        println!("test_my_handler !!!!");
    }
}