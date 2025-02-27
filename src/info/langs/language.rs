use crate::info::info_field::{InfoField, InfoType};
use owo_colors::OwoColorize;
use serde::Serialize;

include!(concat!(env!("OUT_DIR"), "/language.rs"));

const LANGUAGES_BAR_LENGTH: usize = 26;

#[derive(Serialize)]
pub struct LanguageWithPercentage {
    pub language: Language,
    pub percentage: f64,
}

pub struct LanguagesInfo {
    pub languages_with_percentage: Vec<LanguageWithPercentage>,
    pub true_color: bool,
    pub info_color: DynColors,
}

impl LanguagesInfo {
    pub fn new(languages: Vec<(Language, f64)>, true_color: bool, info_color: DynColors) -> Self {
        let languages_with_percentage = languages
            .into_iter()
            .map(|(language, percentage)| LanguageWithPercentage {
                language,
                percentage,
            })
            .collect();
        Self {
            languages_with_percentage,
            true_color,
            info_color,
        }
    }
}

impl std::fmt::Display for LanguagesInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut languages_info = String::new();
        let pad = self.title().len() + 2;
        let color_palette = vec![
            DynColors::Ansi(AnsiColors::Red),
            DynColors::Ansi(AnsiColors::Green),
            DynColors::Ansi(AnsiColors::Yellow),
            DynColors::Ansi(AnsiColors::Blue),
            DynColors::Ansi(AnsiColors::Magenta),
            DynColors::Ansi(AnsiColors::Cyan),
        ];

        let languages: Vec<(String, f64, DynColors)> = {
            let mut iter = self.languages_with_percentage.iter().enumerate().map(
                |(
                    i,
                    &LanguageWithPercentage {
                        language,
                        percentage,
                    },
                )| {
                    let circle_color = if self.true_color {
                        language.get_circle_color()
                    } else {
                        color_palette[i % color_palette.len()]
                    };
                    (language.to_string(), percentage, circle_color)
                },
            );
            if self.languages_with_percentage.len() > 6 {
                let mut languages = iter.by_ref().take(6).collect::<Vec<_>>();
                let other_perc = iter.fold(0.0, |acc, x| acc + x.1);
                languages.push((
                    "Other".to_string(),
                    other_perc,
                    DynColors::Ansi(AnsiColors::White),
                ));
                languages
            } else {
                iter.collect()
            }
        };

        let language_bar: String = languages
            .iter()
            .map(|(_, perc, circle_color)| {
                let bar_width = std::cmp::max(
                    (perc / 100. * LANGUAGES_BAR_LENGTH as f64).round() as usize,
                    1,
                );
                format!("{:<width$}", "".on_color(*circle_color), width = bar_width)
            })
            .collect();

        languages_info.push_str(&language_bar);

        for (i, (language, perc, circle_color)) in languages.iter().enumerate() {
            let formatted_number = format!("{:.*}", 1, perc);
            let circle = "\u{25CF}".color(*circle_color);
            let language_str = format!(
                "{} {} ",
                circle,
                format!("{} ({} %)", language, formatted_number).color(self.info_color)
            );
            if i % 2 == 0 {
                let _ = write!(
                    languages_info,
                    "\n{:<width$}{}",
                    "",
                    language_str,
                    width = pad
                );
            } else {
                languages_info.push_str(language_str.trim_end());
            }
        }
        write!(f, "{}", languages_info)
    }
}

impl InfoField for LanguagesInfo {
    const TYPE: InfoType = InfoType::Languages;

    fn value(&self) -> String {
        self.to_string()
    }

    fn title(&self) -> String {
        let mut title: String = "Language".into();
        if self.languages_with_percentage.len() > 1 {
            title.push('s')
        }
        title
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_languages_info() {
        let languages_info = LanguagesInfo {
            languages_with_percentage: vec![LanguageWithPercentage {
                language: Language::Go,
                percentage: 100_f64,
            }],
            true_color: false,
            info_color: DynColors::Ansi(AnsiColors::White),
        };
        let expected_languages_info = format!(
            "{:<width$}\n{:<pad$}{} {} ",
            "".on_color(DynColors::Ansi(AnsiColors::Red)),
            "",
            "\u{25CF}".color(DynColors::Ansi(AnsiColors::Red)),
            "Go (100.0 %)".color(DynColors::Ansi(AnsiColors::White)),
            width = LANGUAGES_BAR_LENGTH,
            pad = "Language".len() + 2
        );

        assert_eq!(languages_info.value(), expected_languages_info);
    }
}
