use std::time::Duration;

use crate::{
    functions as f,
    structs::{self as s, Account, Trigger},
    traits::CollectionItem,
};
use dotenv::dotenv;

#[tokio::test]
async fn account_exists() {
    dotenv().ok();
    let accounts = f::get_accounts(&f::get_dev_database(&f::get_client().await));
    assert!(
        Account::find_by_username("example username".to_string(), &accounts)
            .await
            .unwrap()
            .is_some()
    );
    assert!(
        Account::find_by_email("sirIusmart@tuta.io".to_string(), &accounts)
            .await
            .unwrap()
            .is_some()
    );
}

#[tokio::test]
async fn new_account_write() {
    dotenv().ok();
    let accounts = f::get_accounts(&f::get_dev_database(&f::get_client().await));

    let username = String::from("example username");
    let pw = "example password";
    let email = String::from("siriusmart@tuta.io");

    let account = s::Account::new(username, pw, email);

    // check passwords
    assert!(account.password_matches("example password"));
    assert!(!account.password_matches("wrong password"));

    account
        .save(&accounts)
        .await
        .expect("cannot save user data");
}

#[tokio::test]
async fn account_verification() {
    dotenv().ok();
    let db = f::get_dev_database(&f::get_client().await);
    let accounts = f::get_accounts(&db);
    let triggers = f::get_triggers(&db);
    let account = s::Account::new(
        String::from("example username"),
        "example password",
        String::from("siriusmart@tuta.io"),
    );

    // check passwords
    assert!(account.password_matches("example password"));
    assert!(!account.password_matches("wrong password"));

    account
        .save(&accounts)
        .await
        .expect("cannot save user data");

    let trigger = Trigger::new_from_action(
        Box::new(account.email_verification()),
        &Duration::from_secs(3600),
    );
    trigger.save(&triggers).await.unwrap();

    assert!(!trigger.is_invalid());
    assert!(Trigger::trigger("not exist", &db).await.is_err());
}
