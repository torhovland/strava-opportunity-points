use strava::api::{AccessToken, RefreshToken};
use strava::athletes::Athlete;

#[tokio::main]
async fn main() {
    let refresh_token_string = "55119...25";
    let client_id_string = "3...7";
    let client_secret_string = "1fb09...b3";

    let refresh_token = RefreshToken::new(
        refresh_token_string.to_string(),
        client_id_string.to_string(),
        client_secret_string.to_string(),
    );

    let token = AccessToken::get_current(&refresh_token)
        .await
        .unwrap_or_else(|error| {
            panic!(
            "Problem getting Strava API access token using refresh token {:?}. The error was: {:?}",
            refresh_token_string, error
        );
        });

    println!("Got access token {:?}", token.access_token);
    println!("Got refresh token {:?}", token.refresh_token);

    let athlete = Athlete::get_current(&token).await.unwrap_or_else(|error| {
        panic!(
            "Problem with Strava API token {:?}. The error was: {:?}",
            token.get(),
            error
        );
    });

    println!("{:?}", athlete);
}
