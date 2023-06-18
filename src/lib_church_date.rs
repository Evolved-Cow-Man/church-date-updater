use chrono::{Datelike, Duration, NaiveDate, Weekday};
use std::{collections::BTreeMap};

#[derive(Copy, Clone)]
pub enum OrdinalLength {
    Short,
    Long
}

pub fn date_ordinal(day: i64, length: OrdinalLength) -> String {
    match length {
        OrdinalLength::Short => {
            match day {
                1 => "1st".to_string(),
                2 => "2nd".to_string(),
                3 => "3rd".to_string(),
                21 => "21st".to_string(),
                22 => "22nd".to_string(),
                23 => "23rd".to_string(),
                31 => "31st".to_string(),
                _ => format!("{day}th")
            }
        }
        OrdinalLength::Long => {
            match day {
                1 => "First".to_string(),
                2 => "Second".to_string(),
                3 => "Third".to_string(),
                4 => "Fourth".to_string(),
                5 => "Fifth".to_string(),
                6 => "Sixth".to_string(),
                7 => "Seventh".to_string(),
                8 => "Eighth".to_string(),
                9 => "Ninth".to_string(),
                10 => "Tenth".to_string(),
                11 => "Eleventh".to_string(),
                12 => "Twelfth".to_string(),
                13 => "Thirteenth".to_string(),
                14 => "Fourteenth".to_string(),
                15 => "Fifteenth".to_string(),
                16 => "Sixteenth".to_string(),
                17 => "Seventeenth".to_string(),
                18 => "Eighteenth".to_string(),
                19 => "Nineteenth".to_string(),
                20 => "Twentieth".to_string(),
                21 => "Twenty-First".to_string(),
                22 => "Twenty-Second".to_string(),
                23 => "Twenty-Third".to_string(),
                24 => "Twenty-Fourth".to_string(),
                25 => "Twenty-Fifth".to_string(),
                26 => "Twenty-Sixth".to_string(),
                27 => "Twenty-Seventh".to_string(),
                28 => "Twenty-Eighth".to_string(),
                29 => "Twenty-Ninth".to_string(),
                30 => "Thirtieth".to_string(),
                31 => "Thirty-First".to_string(),
                _ => day.to_string()
            }
        }
    }
}

pub enum Color {
    White,
    Green,
    Purple,
    Red,
    Yellow,
    Blue,
    Black
}

struct ChurchDate {
    church_text: String,
    liturgical_color: Color
}

pub struct ChurchDateResult {
    pub date: NaiveDate,
    pub text: String,
    pub color: Color
}

pub fn next_church_date(current_date: NaiveDate, length: OrdinalLength) -> ChurchDateResult {
    let mut dates = BTreeMap::new();

    let year = current_date.year();

    //Beginning Sundays of Christmas*
    let mut beginning_sunday_christmas = NaiveDate::from_ymd_opt(year - 1, 12, 26).unwrap();
    while beginning_sunday_christmas.weekday() != Weekday::Sun {
        beginning_sunday_christmas += Duration::days(1)
    }
    for week in 0..=1 {
        let ordinal_week = date_ordinal(week + 1, length);
        let beginning_sunday_christmas_value = ChurchDate {
            church_text: format!("{ordinal_week} Sunday of Christmas"),
            liturgical_color: Color::White
        };
        let beginning_sunday_christmas_date = beginning_sunday_christmas + Duration::weeks(week);
        dates.insert(beginning_sunday_christmas_date, beginning_sunday_christmas_value);
    };

    //Epiphany of Our Lord*
    let epiphany_date = NaiveDate::from_ymd_opt(year, 1, 6).unwrap();
    let epiphany_value = ChurchDate {
        church_text: "Epiphany of Our Lord".to_string(),
        liturgical_color: Color::White
    };
    dates.insert(epiphany_date, epiphany_value);

    //Baptism of Our Lord*
    let mut baptism_of_our_lord_date = NaiveDate::from_ymd_opt(year, 1, 7).unwrap();
    while baptism_of_our_lord_date.weekday() != Weekday::Sun {
        baptism_of_our_lord_date += Duration::days(1)
    }
    let baptism_of_our_lord_value = ChurchDate {
        church_text: "Baptism of Our Lord".to_string(),
        liturgical_color: Color::White
    };
    dates.insert(baptism_of_our_lord_date, baptism_of_our_lord_value);

    //Sundays after Epiphany*
    for week in 1..=7 {
        let ordinal_week = date_ordinal(week + 1, length);
        let sunday_after_epiphany_value = ChurchDate {
            church_text: format!("{ordinal_week} Sunday after Epiphany"),
            liturgical_color: Color::Green
        };
        let sunday_after_epiphany_date = baptism_of_our_lord_date + Duration::weeks(week);
        dates.insert(sunday_after_epiphany_date, sunday_after_epiphany_value);
    };

    //get easter
    let easter_date = bdays::easter::easter_naive_date(year).expect("Should be a valid year for generating easter.");

    //Transfiguration of Our Lord*
    let transfiguration_date = easter_date - Duration::weeks(7);
    let transfiguration_value = ChurchDate {
        church_text: "Transfiguration of Our Lord".to_string(),
        liturgical_color: Color::White
    };
    dates.insert(transfiguration_date, transfiguration_value);

    //Ash Wednesday*
    let ash_wednesday_date = easter_date - Duration::weeks(6) - Duration::days(4);
    let ash_wednesday_value = ChurchDate {
        church_text: "Ash Wednesday".to_string(),
        liturgical_color: Color::Purple
    };
    dates.insert(ash_wednesday_date, ash_wednesday_value);

    //Sundays / Wednesdays in Lent*
    let lent = easter_date - Duration::weeks(6);
    for week in 0..=4 {
        let ordinal_week = date_ordinal(week + 1, length);
        let lent_sundays_value = ChurchDate {
            church_text: format!("{ordinal_week} Sunday in Lent"),
            liturgical_color: Color::Purple
        };
        let lent_sundays_date = lent + Duration::weeks(week);
        dates.insert(lent_sundays_date, lent_sundays_value);

        let lent_wednesdays_value = ChurchDate {
            church_text: format!("{ordinal_week} Wednesday in Lent"),
            liturgical_color: Color::Purple
        };
        let lent_wednesdays_date = lent + Duration::weeks(week) + Duration::days(3);
        dates.insert(lent_wednesdays_date, lent_wednesdays_value);
    };

    //Palm Sunday*
    let palm_sunday_date = easter_date - Duration::weeks(1);
    let palm_sunday_value = ChurchDate {
        church_text: "Palm Sunday".to_string(),
        liturgical_color: Color::Purple
    };
    dates.insert(palm_sunday_date, palm_sunday_value);

    //Maundy Thursday*
    let maundy_thursday_date = easter_date - Duration::days(3);
    let maundy_thursday_value = ChurchDate {
        church_text: "Maundy Thursday".to_string(),
        liturgical_color: Color::Red
    };
    dates.insert(maundy_thursday_date, maundy_thursday_value);

    //Good Friday*
    let good_friday_date = easter_date - Duration::days(2);
    let good_friday_value = ChurchDate {
        church_text: "Good Friday".to_string(),
        liturgical_color: Color::Black
    };
    dates.insert(good_friday_date, good_friday_value);

    //Easter Sunday*
    let easter_value = ChurchDate {
        church_text: "Easter Sunday".to_string(),
        liturgical_color: Color::Yellow
    };
    dates.insert(easter_date, easter_value);

    //Sundays of Easter*
    for week in 1..=6 {
        let ordinal_week = date_ordinal(week + 1, length);
        let easter_sundays_value = ChurchDate {
            church_text: format!("{ordinal_week} Sunday of Easter"),
            liturgical_color: Color::White
        };
        let easter_sundays_date = easter_date + Duration::weeks(week);
        dates.insert(easter_sundays_date, easter_sundays_value);
    };

    //Day of Pentecost*
    let pentecost_date = easter_date + Duration::weeks(7);
    let pentecost_value = ChurchDate {
        church_text: "Day of Pentecost".to_string(),
        liturgical_color: Color::Red
    };
    dates.insert(pentecost_date, pentecost_value);

    //The Holy Trinity*
    let holy_trinity_date = easter_date + Duration::weeks(8);
    let holy_trinity_value = ChurchDate {
        church_text: "The Holy Trinity".to_string(),
        liturgical_color: Color::White
    };
    dates.insert(holy_trinity_date, holy_trinity_value);

    //Sundays of Pentecost*
    for week in 9..=33 {
        let ordinal_week = date_ordinal(week - 7, length);
        let pentecost_sundays_value = ChurchDate {
            church_text: format!("{ordinal_week} Sunday of Pentecost"),
            liturgical_color: Color::Green
        };
        let pentecost_sundays_date = easter_date + Duration::weeks(week);
        dates.insert(pentecost_sundays_date, pentecost_sundays_value);
    };

    //Christ the King Sunday*
    let mut christ_the_king_date = NaiveDate::from_ymd_opt(year, 12, 24).unwrap() - Duration::weeks(4);
    while christ_the_king_date.weekday() != Weekday::Sun {
        christ_the_king_date -= Duration::days(1)
    }
    let christ_the_king_value = ChurchDate {
        church_text: "Christ the King Sunday".to_string(),
        liturgical_color: Color::White
    };
    dates.insert(christ_the_king_date, christ_the_king_value);

    //Sundays of Advent*
    for week in 1..=4 {
        let ordinal_week = date_ordinal(week, length);
        let advent_value = ChurchDate {
            church_text: format!("{ordinal_week} Sunday of Advent"),
            liturgical_color: Color::Blue
        };
        let advent_date = christ_the_king_date + Duration::weeks(week);
        dates.insert(advent_date, advent_value);
    };

    //Christmas Eve*
    let christmas_eve_date = NaiveDate::from_ymd_opt(year, 12, 24).unwrap();
    let christmas_eve_value = ChurchDate {
        church_text: "Christmas Eve".to_string(),
        liturgical_color: Color::White
    };
    dates.insert(christmas_eve_date, christmas_eve_value);

    //Christmas Day*
    let christmas_date = NaiveDate::from_ymd_opt(year, 12, 25).unwrap();
    let christmas_value = ChurchDate {
        church_text: "Christmas Day".to_string(),
        liturgical_color: Color::White
    };
    dates.insert(christmas_date, christmas_value);

    //First Sunday of Christmas*
    let mut first_sunday_christmas_date = NaiveDate::from_ymd_opt(year, 12, 26).unwrap();
    while first_sunday_christmas_date.weekday() != Weekday::Sun {
        first_sunday_christmas_date += Duration::days(1)
    }
    let first_sunday_christmas_value = ChurchDate {
        church_text: format!("{} Sunday of Christmas", date_ordinal(1, length)),
        liturgical_color: Color::White
    };
    dates.insert(first_sunday_christmas_date, first_sunday_christmas_value);

    //special days:

    //Reformation Sunday* (the sunday on or before October 31st)
    let mut reformation_date = NaiveDate::from_ymd_opt(year, 10, 31).unwrap();
    while reformation_date.weekday() != Weekday::Sun {
        reformation_date -= Duration::days(1)
    }
    let reformation_value = ChurchDate {
        church_text: "Reformation Sunday".to_string(),
        liturgical_color: Color::Red
    };
    dates.insert(reformation_date, reformation_value);

    //All Saints Sunday* (the Sunday on or after November 1st)
    let reformation_date = reformation_date + Duration::weeks(1);
    let reformation_value = ChurchDate {
        church_text: "All Saints Sunday".to_string(),
        liturgical_color: Color::White
    };
    dates.insert(reformation_date, reformation_value);

    //Confirmation Sunday* (whenever it's convenient)

    //find next date and return value:
    let mut result = None;

    for (church_date, church_value) in dates {
        if church_date >= current_date {
            result = Some(
                    ChurchDateResult {
                    date: church_date,
                    text: church_value.church_text,
                    color: church_value.liturgical_color
                }
            );
            break
        }
    }

    return result.expect("Current date should be within generated dates.");
}
