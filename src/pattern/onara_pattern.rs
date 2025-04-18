use super::emoji_tag;
use super::emoji_type::EMOJI_TYPE;
use super::ojisan_emotion::OjisanEmotion;
use super::onara_messages;
use super::tag_type::TAG_TYPE;
use super::tags;

use kanaria::string::UCSStr;
use rand::seq::SliceRandom;
use rand::Rng;
use regex::Regex;

#[derive(Clone)]
pub struct OnaraPattern {
    pattern: Vec<OjisanEmotion>,
}

impl OnaraPattern {
    pub fn new(pattern: Vec<OjisanEmotion>) -> Self {
        OnaraPattern { pattern }
    }

    pub fn get_message(self, target: String, emoji_num: usize) -> String {
        let mut result = "".to_string();
        for emotion in self.pattern {
            let mut tmp = onara_messages::select_template(emotion).to_string();
            if emotion != OjisanEmotion::GREETING {
                let mut rng = rand::rng();
                let n: i32 = rng.random_range(0..2);
                tmp = self::OnaraPattern::katakana_katsuyou(tmp, n);
            }
            result = format!("{}{}", result, tmp);
        }
        result = OnaraPattern::convert_tags(result);
        result = OnaraPattern::convert_emoji(result, emoji_num);

        result = OnaraPattern::convert_target(result, target);
        result
    }

    fn convert_tags(mut result: String) -> String {
        for val in (*TAG_TYPE).values() {
            let count = result.split(&format!("{{{}}}", val)).count() - 1;
            for _ in 0..count {
                let mut rng = rand::rng();
                let mut selected_tags = tags::select_tags(val);
                selected_tags.shuffle(&mut rng);
                result = result.replace(&format!("{{{}}}", val), selected_tags[0]);
            }
        }
        result
    }

    fn convert_emoji(mut result: String, emoji_num: usize) -> String {
        for val in (*EMOJI_TYPE).values() {
            let count = result.split(&format!("{{{}}}", val)).count() - 1;
            for _ in 0..count {
                let mut rng = rand::rng();
                let mut emoji_list = emoji_tag::select_tags(val);

                let mut n: usize = rng.random_range(0..emoji_num);

                emoji_list.shuffle(&mut rng);
                if n > emoji_list.len() {
                    n = emoji_list.len() - 1;
                }
                let content = &*emoji_list.get(0..n).unwrap().join("");
                result = result.replace(&format!("{{{}}}", val), content);
            }
        }
        result
    }
    fn convert_target(result: String, target: String) -> String {
        result.replace("{TARGET_NAME}", &*target)
    }

    // カタカナ活用を適用する
    fn katakana_katsuyou(message: String, number: i32) -> String {
        let re = Regex::new(
            &*(r"^(.+)(\p{Hiragana}{".to_string() + &*number.to_string() + r"})([^\p{Hiragana}]*)"),
        )
        .unwrap();
        let hiraganas = re.captures(&*message);
        let hiraganas = match hiraganas {
            None => return message,
            Some(n) => n,
        };
        if hiraganas.len() != 4 {
            return message;
        }
        hiraganas.get(1).unwrap().as_str().to_string()
            + &*UCSStr::from_str(hiraganas.get(2).unwrap().as_str())
                .katakana()
                .to_string()
            + hiraganas.get(3).unwrap().as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_katakana_katsuyou1() {
        let expected = "なんちゃッテ".to_string();
        let actual = OnaraPattern::katakana_katsuyou("なんちゃって".to_string(), 2);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_katakana_katsuyou2() {
        let expected = "どうしちゃったノカナ{EMOJI_POS}".to_string();
        let actual =
            OnaraPattern::katakana_katsuyou("どうしちゃったのかな{EMOJI_POS}".to_string(), 3);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_katakana_katsuyou3() {
        let expected = "どうしちゃったのかな{EMOJI_POS}".to_string();
        let actual =
            OnaraPattern::katakana_katsuyou("どうしちゃったのかな{EMOJI_POS}".to_string(), 0);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_katakana_katsuyou4() {
        let expected = "東西南北{EMOJI_POS}".to_string();
        let actual = OnaraPattern::katakana_katsuyou("東西南北{EMOJI_POS}".to_string(), 2);
        assert_eq!(actual, expected)
    }
}
