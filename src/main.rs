extern crate strava;

fn main() {
    use strava::athletes::Athlete;
    use strava::api::AccessToken;
    
    let token_string = "<my token>";
    let token = AccessToken::new(token_string.to_string());    
    let athlete = Athlete::get_current(&token).unwrap_or_else(|error| {
        panic!("Problem with Strava API token {:?}. The error was: {:?}", token_string, error);
    });
    
    println!("{:?}", athlete);
}