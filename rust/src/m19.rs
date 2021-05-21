/// You are given the following information, but you may prefer to do some research for yourself.
///
/// 1 Jan 1900 was a Monday.
/// Thirty days has September,
/// April, June and November.
/// All the rest have thirty-one,
/// Saving February alone,
/// Which has twenty-eight, rain or shine.
/// And on leap years, twenty-nine.
/// A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
///
/// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
///
/// ```rust
/// use self::project_euler::m19::how_many_sundays_fell_on_the_first_of_the_month;
/// assert_eq!(how_many_sundays_fell_on_the_first_of_the_month(), 171);
/// ```
pub fn how_many_sundays_fell_on_the_first_of_the_month() -> u32 {
    let length_of_february = |year: u32| -> u32 {
        if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
            29
        } else {
            28
        }
    };

    // to think, 0=mon 1=tue 2=wed 3=thu 4=fri 5=sat 6=sun
    // the day 0, 1900-1-1 is mon (0)
    // in 1900, 365?=[31, 28?, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    // th day      =[0, 0+31, 0+31+31, 0+31+31+30..]
    // is Sunday   =[0%7==6, 0+31%7==6..]
    let mut th_day = 0u64;
    let mut sunday_counter = 0u32;
    for y in 1900..=2000u32 {
        for m in 1..=12 {
            if y == 1900 && m == 1 {
                continue;
            }
            // adds the length of the previous month
            th_day += match m {
                1 | 2 | 4 | 6 | 8 | 9 | 11 => 31,
                3 => length_of_february(y) as u64,
                5 | 7 | 10 | 12 => 30,
                _ => panic!(),
            };
            // don't count Sundays in 1900,
            // count from 1901-Jan-1 to 2000-Dec-1 inclusive.
            if y != 1900 && th_day % 7 == 6 {
                sunday_counter += 1;
            }
        }
    }
    sunday_counter
}

fn length_of_february(year: u16) -> u8 {
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        29
    } else {
        28
    }
}

struct FirstDayOfMonth {
    year: u16,
    month: u8,
    day_count: u64,
    sunday_count: u32,
}

impl FirstDayOfMonth {
    fn new() -> Self {
        FirstDayOfMonth {
            year: 1900,
            month: 1,
            day_count: 0,
            sunday_count: 0,
        }
    }
    fn is_sunday(&self) -> bool {
        self.day_count % 7 == 6
    }
    fn next_month(&mut self) {
        self.day_count += match self.month {
            2 => length_of_february(self.year) as u64,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        };
        if self.month == 12 {
            self.year += 1;
            self.month = 1;
        } else {
            self.month += 1;
        }

        if self.year != 1900 && self.is_sunday() {
            self.sunday_count += 1;
        }
    }
}

// 2.9277 us
/// ```rust
/// use self::project_euler::m19::how_many_sundays_fell_on_the_first_of_the_month_2;
/// assert_eq!(how_many_sundays_fell_on_the_first_of_the_month_2(), 171);
/// ```
pub fn how_many_sundays_fell_on_the_first_of_the_month_2() -> u32 {
    let mut cal = FirstDayOfMonth::new();
    while !(cal.year == 2000 && cal.month == 12) {
        cal.next_month();
    }
    cal.sunday_count
}

#[derive(PartialEq)]
enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

fn zeller_congruence(mut y: u32, mut m: u32, d: u32) -> Weekday {
    if m == 1 || m == 2 {
        m += 12;
        y -= 1;
    }
    let yd = y / 100;
    let ym = y % 100;
    match (d + (26 * (m + 1)) / 10 + ym + ym / 4 + yd / 4 + 5 * yd) % 7 {
        0 => Weekday::Sat,
        1 => Weekday::Sun,
        2 => Weekday::Mon,
        3 => Weekday::Tue,
        4 => Weekday::Wed,
        5 => Weekday::Thu,
        6 => Weekday::Fri,
        _ => panic!(),
    }
}

// 8.2 us
/// ```rust
/// use self::project_euler::m19::how_many_sundays_fell_on_the_first_of_the_month_zeller;
/// assert_eq!(how_many_sundays_fell_on_the_first_of_the_month_zeller(), 171);
/// ```
pub fn how_many_sundays_fell_on_the_first_of_the_month_zeller() -> u32 {
    let mut sum = 0u32;
    for y in 1901u32..=2000 {
        for m in 1u32..=12 {
            let weekday = zeller_congruence(y, m, 1);
            if weekday == Weekday::Sun {
                sum += 1;
            }
        }
    }
    sum
}
