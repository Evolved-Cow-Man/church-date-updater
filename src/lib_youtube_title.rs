extern crate google_youtube3 as youtube3;
extern crate hyper;
extern crate hyper_rustls;

use std::default::Default;
use youtube3::api::{LiveBroadcast, LiveBroadcastSnippet};
use youtube3::{oauth2, Error, YouTube};

use dialoguer::{theme::ColorfulTheme, Confirm};

#[tokio::main]
pub async fn update_youtube_title(new_title: String) {
    let client_secret = "secret.json"; // enter file path to API client secret here
    let secret = oauth2::read_application_secret(client_secret)
        .await
        .expect(client_secret);

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("tokencache.json")
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
                .build(),
        ),
        auth,
    );

    /*
     * Get Broadcast ID
     */
    //I need to initialize something here, but this should in no way be used
    let mut broadcast_id = String::new();

    let mut retry = true;

    //Loop over it until something sets 'retry' to false.
    while retry {
        //Check for a current YouTube livestream
        let broadcasts_list_active = hub
            .live_broadcasts()
            .list(&vec!["id".into()])
            .broadcast_status("active")
            .doit()
            .await
            .unwrap()
            .1;

        //If there is a current YouTube livestream, use it. if there are none, it will return an empty vector so unwrap can be used. Set retry to false
        let mut id_vec = broadcasts_list_active.items.unwrap();

        #[allow(clippy::if_not_else)]
        if !id_vec.is_empty() {
            retry = false;
            println!("Using active stream ID.");
            let id_ref = id_vec[0].id.as_ref();
            broadcast_id = String::from(id_ref.unwrap());
        }
        //Check for upcoming live stream
        else {
            let broadcasts_list_upcoming = hub
                .live_broadcasts()
                .list(&vec!["id".into()])
                .broadcast_status("upcoming")
                .doit()
                .await
                .unwrap()
                .1;

            id_vec = broadcasts_list_upcoming.items.unwrap();

            if !id_vec.is_empty() {
                retry = false;
                println!("Using upcoming ID.");
                let id_ref = id_vec[0].id.as_ref();
                broadcast_id = String::from(id_ref.unwrap());
            } else {
                //Ask to try again to find the YouTube ID
                if Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Couldn't find any YouTube IDs. Try dismissing anything in the YouTube studio before trying again. Would you like to try again?".to_string())
                    .default(true)
                    .interact()
                    .unwrap()
                {
                    //Nothing needs to be done to retry
                } else {
                    println!("Not updating the YouTube title.");
                    return;
                }
            }
        }
    }

    /*
     * Update title
     */
    let snippet = LiveBroadcastSnippet {
        title: Some(new_title),
        ..Default::default()
    };

    let req = LiveBroadcast {
        id: Some(broadcast_id),
        snippet: Some(snippet),
        ..Default::default()
    };

    let result = hub
        .live_broadcasts()
        .update(req)
        .add_part("id")
        .add_part("snippet")
        .doit()
        .await;

    match result {
        Err(e) => match e {
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => println!("Unable to update the YouTube title: {e}"),
        },
        Ok(_) => println!("Successfully updated the YouTube title."),
    }
}
