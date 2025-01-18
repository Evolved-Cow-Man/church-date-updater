# Church Date Updater
A program used to generate liturgical dates and colors for use in OBS/YouTube.

## Setup
1. Clone this repository.
2. Make sure you already have [Cargo](https://www.rust-lang.org/) installed.
3. Run `cargo build -r`.
4. Put `/target/release/church-date-updater.exe` wherever you prefer.
5. Put `/pics`, `lower_data`, and `church_date.html` in the same directory as your binary.
6. If you want YouTube live title updating, obtain a key from [YouTube Data API](https://developers.google.com/youtube/v3). Name it `secret.json` and place it in the same directory as your binary.
7. `pics/Current_Color.png` and `church_date.html` can be used in OBS.
8. Run `church-date-updater.exe`.
