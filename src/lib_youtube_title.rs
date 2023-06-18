extern crate hyper;
extern crate hyper_rustls;
extern crate google_youtube3 as youtube3;

use std::default::Default;
use youtube3::{YouTube, Error, oauth2};
use youtube3::api::{LiveBroadcast, LiveBroadcastSnippet};

use dialoguer::{theme::ColorfulTheme, Confirm};

#[tokio::main]
pub async fn update_youtube_title(new_title: String) {
    let client_secret = "secret.json"; // enter file path to API client secret here
    let secret = oauth2::read_application_secret(client_secret)
        .await
        .expect(client_secret);

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect
    ).persist_tokens_to_disk("tokencache.json")
        .build()
        .await
        .unwrap();

    let hub = YouTube::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build()
        ),
        auth
    );

    /*
     * Get Broadcast ID
     */
    //I need to initialize something here, but this should in no way be used
    let mut broadcast_id = String::from(" ");

    let mut retry = true;

    //Loop over it untill something sets retry to false. don't set to false to retry
    while retry {
        //Check to see if there is a current YouTube livestream
        let broadcasts_list_active = hub.live_broadcasts()
            .list(&vec!["id".into()]).broadcast_status("active").doit().await.unwrap().1;

        broadcast_id = match broadcasts_list_active.items {
            None => panic!("Error: no live broadcasts found!"),
            Some(vec) => {
                if vec.len() > 0 {
                    retry = false;
                    println!("Using active stream ID.");
                    let id_ref = vec[0].id.as_ref();
                    String::from(
                        id_ref.expect("Error: no broadcast ID found!").as_str()
                    )
                }

                //If not, check for upcomeing live stream
                else {
                    let broadcasts_list_upcoming = hub.live_broadcasts()
                        .list(&vec!["id".into()]).broadcast_status("upcoming").doit().await.unwrap().1;

                    match broadcasts_list_upcoming.items {
                        None => panic!("Error: no live broadcasts found!"),
                        Some(vec) => {
                            if vec.len() > 0 {
                                retry = false;
                                println!("Using upcoming ID.");
                                let id_ref = vec[0].id.as_ref();
                                String::from(
                                    id_ref.expect("Error: no broadcast ID found!").as_str()
                                )
                            } else {
                                //Ask to try again to find the YouTube ID
                                if Confirm::with_theme(&ColorfulTheme::default())
                                    .with_prompt(format!("Couldn't find any YouTube IDs. Try dissmissing anything in the YouTube studio before trying again. Would you like to try again?"))
                                    .default(true)
                                    .interact()
                                    .unwrap()
                                {
                                    //I need to return something here, but this should in no way be used.
                                    String::from(" ")
                                } else {
                                    //I need to return something here, but this should in no way be used.
                                    String::from(" ")
                                }
                            }
                        }
                    }
                }
            }
        };
    }

    /*
     * Update title
     */
    let mut snippet = LiveBroadcastSnippet::default();
    snippet.title = Some(new_title);

    let mut req = LiveBroadcast::default();
    req.id = Some(broadcast_id);
    req.snippet = Some(snippet);

    let result = hub.live_broadcasts().update(req)
        .add_part("id")
        .add_part("snippet")
        .doit().await;

    match result {
        Err(e) => match e {
            Error::HttpError(_)
                |Error::Io(_)
                |Error::MissingAPIKey
                |Error::MissingToken(_)
                |Error::Cancelled
                |Error::UploadSizeLimitExceeded(_, _)
                |Error::Failure(_)
                |Error::BadRequest(_)
                |Error::FieldClash(_)
                |Error::JsonDecodeError(_, _) => println!("Unable to update the YouTube title: {}", e),
        },
        Ok(_) => println!("Successfully updated the YouTube title."),
    }
}
