use mmcli::User;

#[test]
fn parse() {
    let s = "{\"id\": \"hello\", \"email_verified\": \"true\"}";
    let u: User = serde_json::from_str(s).unwrap();
    assert_eq!(u.id.unwrap(), "hello");
    assert!(u.email_verified.unwrap());
}
