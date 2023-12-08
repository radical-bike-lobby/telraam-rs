use serde::Deserialize;

#[derive(Deserialize)]
pub struct WelcomeResponse {
    pub msg: String,
}

#[test]
fn test_deserialize_welcome() {
    let json = r#"
    {
        "msg": "hello! Telraam server 2.0 is up and running"
    }
    "#;

    let welcome = serde_json::from_str::<WelcomeResponse>(json).expect("failed to parse json");
    assert_eq!("hello! Telraam server 2.0 is up and running", welcome.msg);
}
