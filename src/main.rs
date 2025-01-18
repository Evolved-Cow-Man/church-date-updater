use chrono::{Datelike, Local};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

mod lib_church_date;
use lib_church_date::Color;
use lib_church_date::OrdinalLength::{Long, Short};
use lib_church_date::{date_ordinal, next_church_date};

mod lib_youtube_title;
use lib_youtube_title::update_youtube_title;

fn main() {
    let current_date = Local::now().date_naive();

    let current_date_formated = current_date.format("%m-%d-%y").to_string();

    let current_month_day = current_date.day();

    let current_month_formated = current_date.format("%B");

    let ordinal_month_short = date_ordinal(current_month_day.into(), Short);

    /*
     * OBS church date
     */
    let obs_church_date = next_church_date(current_date, Short).text;

    let mut obs_church_date_formated =
        format!("{current_month_formated} {ordinal_month_short}, {obs_church_date}");

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("The church date used for OBS looks like this: '{obs_church_date_formated}'. Does this look okay?"))
        .default(true)
        .interact()
        .unwrap()
    {
    } else {
        obs_church_date_formated = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("What should it look like?")
            .with_initial_text(obs_church_date_formated)
            .interact_text()
            .unwrap();
    }
    println!("Okay, using '{obs_church_date_formated}' for OBS.");

    let mut obs_lower_list = vec![obs_church_date_formated];

    let youtube_church_date = next_church_date(current_date, Long).text;

    let mut youtube_church_date_formated;

    /*
     * Sermon title
     */
    println!("Sermon titles have \"quotes\" added to them automatically.");
    println!("The title should have already been carefully considered by the person who wrote the sermon.");
    println!("Examples include: 'All Are Welcome' or '...And Peter'");
    println!(
        "Sermon titles are {}",
        "not names, dates, or other information."
            .bold()
            .underline()
            .red()
    );
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to add a sermon title?")
        .default(true)
        .interact()
        .unwrap()
    {
        let sermon_title: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Okay, what is the sermon title?")
            .interact_text()
            .unwrap();
        println!("Okay, using '{sermon_title}' for sermon title.");
        youtube_church_date_formated =
            format!("\"{sermon_title}\" - {current_date_formated} - {youtube_church_date}");
        obs_lower_list.push(format!("\"{sermon_title}\""));
    } else {
        println!("Okay, not using a sermon title.");
        youtube_church_date_formated = format!("{current_date_formated} - {youtube_church_date}");
    }

    /*
     * Extra Text
     */
    let mut extra_text_retry = true;

    let mut extra_text_count = 0;

    while extra_text_retry {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to add more text to the lower third for OBS?")
            .default(false)
            .interact()
            .unwrap()
        {
            let extra_lower_text = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Okay, what else would you like to add?")
                .interact_text()
                .unwrap();
            println!("Okay, adding '{extra_lower_text}' to the OBS lower third.");

            // add the first extra text to the title format
            extra_text_count += 1;
            if extra_text_count == 1 {
                youtube_church_date_formated =
                    format!("{extra_lower_text} - {current_date_formated} - {youtube_church_date}");
            }

            obs_lower_list.push(extra_lower_text);
        } else {
            println!("Okay, not adding extra text.");
            extra_text_retry = false;
        }
    }

    //Write church text for OBS to file
    #[allow(clippy::items_after_statements)]
    fn write_lower_data(obs_lower_list: Vec<String>) -> Result<(), std::io::Error> {
        let file = File::create("lower_data.txt")?;
        let mut writer = BufWriter::new(file);

        for item in obs_lower_list {
            writeln!(writer, "{item}")?;
        }

        writer.flush()?;
        Ok(())
    }

    match write_lower_data(obs_lower_list) {
        Ok(()) => println!("Successfully changed the OBS lower third."),
        Err(err) => {
            println!("Unable to change the OBS lower third: {err}");
            println!("You can still continue with the rest of the setup.");
        }
    }

    /*
     * OBS liturgical color
     */
    let possible_colors = &[
        "White".white(),
        "Green".green(),
        "Purple".purple(),
        "Red".red(),
        "Yellow".yellow(),
        "Blue".blue(),
        "Black".white(), //some terminals will display black as background color
        "None".white(),
    ];

    let possible_strings = &[
        "White", "Green", "Purple", "Red", "Yellow", "Blue", "Black", "None",
    ];

    let suggested_color = next_church_date(current_date, Short).color;

    let suggested_color_value = match suggested_color {
        Color::White => 0,
        Color::Green => 1,
        Color::Purple => 2,
        Color::Red => 3,
        Color::Yellow => 4,
        Color::Blue => 5,
        Color::Black => 6,
    };

    let mut color_string = &possible_colors[suggested_color_value];

    let mut color_string_regular = possible_strings[suggested_color_value];

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "The liturgical color used for OBS is: '{color_string}'. Does this look okay?"
        ))
        .default(true)
        .interact()
        .unwrap()
    {
        println!("Okay, using '{color_string}'.");
    } else {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What should it be?")
            .default(suggested_color_value)
            .items(&possible_colors[..])
            .interact()
            .unwrap();
        color_string = &possible_colors[selection];
        println!("Okay, using '{color_string}'.");

        color_string_regular = possible_strings[selection];
    }

    let old_file = format!("pics/{color_string_regular}.png");

    let new_file = "pics/Current_Color.png";

    //you need to remove the file first to have OBS know that something changed
    match fs::remove_file(new_file) {
        Ok(()) => {
            // Don't need to print anything
        }
        Err(err) => {
            println!("Unable to remove old color file for OBS: {err}");
            println!("This could be because the file does not exist, setup will continue.");
        }
    }

    std::thread::sleep(std::time::Duration::from_millis(200)); //wait becuase some computers run it too fast, and OBS does not see the new file change
    match fs::copy(old_file, new_file) {
        Ok(_) => println!("Successfully changed color for OBS."),
        Err(err) => {
            println!("Unable to change color for OBS: {err}");
            println!("You can still continue with the rest of the setup.");
        }
    }

    /*
     * Update YouTube api
     */
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("The title used for YouTube looks like this: '{youtube_church_date_formated}'. Does this look okay?"))
        .default(true)
        .interact()
        .unwrap()
    {

    } else {
        youtube_church_date_formated = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("What should it look like?")
            .with_initial_text(youtube_church_date_formated)
            .interact_text()
            .unwrap();
    }
    println!("Okay, using '{youtube_church_date_formated}'.");

    //test for secret.json
    if fs::metadata("secret.json").is_ok() {
        update_youtube_title(youtube_church_date_formated);
    } else {
        println!("'secret.json' does not exist. Not updating youtube title.");
    }

    //wait a little bit so they can read the text
    let wait_time = std::time::Duration::from_secs(3);
    std::thread::sleep(wait_time);
}
