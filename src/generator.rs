use super::pattern::onara::Onara;
use super::pattern::onara_pattern;

use lindera::{
    dictionary::{load_dictionary_from_kind, DictionaryKind},
    segmenter::Segmenter,
    tokenizer::Tokenizer,
};
use lindera_dictionary::mode::Mode;
use rand::{rng, Rng};

pub struct Generator {}

#[derive(Clone)]
struct PunctuationConfig {
    target_hinshis: Vec<&'static str>,
    rate: i32,
}

impl Generator {
    pub fn get_message(
        target_name: Option<String>,
        emoji_num: usize,
        punctuation_level: usize,
    ) -> String {
        let pconfigs: Vec<PunctuationConfig> = vec![
            PunctuationConfig {
                target_hinshis: vec![],
                rate: 0,
            },
            PunctuationConfig {
                target_hinshis: vec!["助動詞"],
                rate: 30,
            },
            PunctuationConfig {
                target_hinshis: vec!["助動詞", "助詞"],
                rate: 60,
            },
            PunctuationConfig {
                target_hinshis: vec!["助動詞", "助詞"],
                rate: 100,
            },
        ];

        let level = punctuation_level;
        let target = target_name.unwrap_or_else(Generator::get_random_firstname)
            + Generator::get_random_name_suffix();
        let pattern: onara_pattern::OnaraPattern = Onara::select_pattern();
        let message = pattern.get_message(target, emoji_num);
        Generator::insert_punctuations(message, pconfigs[level].clone())
    }

    fn get_random_firstname() -> String {
        let name = gimei::female();
        let mut rng = rng();
        let n: i32 = rng.random_range(0..2);
        match n {
            0 => name.first.kanji,
            1 => name.first.katakana,
            2 => name.first.hiragana,
            _ => name.first.katakana,
        }
    }

    fn get_random_name_suffix() -> &'static str {
        let mut rng = rng();
        let n: i32 = rng.random_range(0..99);
        if n < 5 {
            // たまに呼び捨てにするおじさん
            ""
        } else if n < 20 {
            // 時に「◯◯チャン」とカタカナにしてくるのも、おじさんの常套手段だ。
            "チャン"
        } else if n < 40 {
            // "「〇〇チャン」をさらに半角で表現する、そんなおじさんもいる"
            "ﾁｬﾝ"
        } else {
            "ちゃん"
        }
    }
    fn insert_punctuations(message: String, config: PunctuationConfig) -> String {
        if config.rate == 0 {
            return message;
        }
        let mut rng = rng();
        let mut result: String = "".to_string();
        // おじさんの文句の形態素解析に使われるなんて可哀そうなライブラリだな
        let dictionary = load_dictionary_from_kind(DictionaryKind::IPADIC)
            .expect("Failed to load the dictionary");
        let tokenizer = Tokenizer::new(Segmenter {
            mode: Mode::Normal,
            dictionary,
            user_dictionary: None,
        });
        let tokens = tokenizer.tokenize(&*message).expect("Failed to tokenize");
        for mut token in tokens {
            let features = token.details();
            let mut hinshi_flag = false;
            for hinshi in config.target_hinshis.clone() {
                if hinshi == features[0] {
                    hinshi_flag = true;
                    break;
                }
            }
            if hinshi_flag && rng.random_range(1..100) <= config.rate {
                result += &token.text;
                result += "、"
            } else {
                result += &token.text
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_punctuations1() {
        let expected = "どうしちゃったのかな".to_string();
        let pconfig = PunctuationConfig {
            target_hinshis: vec!["助動詞", "名詞"],
            rate: 0,
        };
        let actual = Generator::insert_punctuations("どうしちゃったのかな".to_string(), pconfig);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_insert_punctuations2() {
        let expected = "どうしちゃった、のかな".to_string();
        let pconfig = PunctuationConfig {
            target_hinshis: vec!["助動詞"],
            rate: 100,
        };
        let actual = Generator::insert_punctuations("どうしちゃったのかな".to_string(), pconfig);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_insert_punctuations3() {
        let expected = "どうしちゃった、のか、な、".to_string();
        let pconfig = PunctuationConfig {
            target_hinshis: vec!["助動詞", "助詞"],
            rate: 100,
        };
        let actual = Generator::insert_punctuations("どうしちゃったのかな".to_string(), pconfig);
        assert_eq!(expected, actual);
    }
}
