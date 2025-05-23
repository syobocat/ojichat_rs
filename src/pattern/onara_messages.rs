use super::ojisan_emotion::OjisanEmotion;
use rand::prelude::*;

pub fn select_template(emotion: OjisanEmotion) -> &'static str {
    let mut messages: Vec<&str>;
    match emotion {
        OjisanEmotion::GREETING => {
            messages = vec![
                "{TARGET_NAME}{EMOJI_POS}",
                "{TARGET_NAME}、お疲れ様〜{EMOJI_POS}",
                "{TARGET_NAME}、オハヨウ〜{EMOJI_POS}",
                "{TARGET_NAME}、おっは〜{EMOJI_POS}",
                "{TARGET_NAME}、オッハー{EMOJI_POS}",
                "{TARGET_NAME}、オハヨー{EMOJI_POS}",
                "{TARGET_NAME}、お早う{EMOJI_POS}",
                "{TARGET_NAME}、ヤッホー{EMOJI_POS}何してるのかい{EMOJI_ASK}",
                "{TARGET_NAME}、今日もお仕事かな{EMOJI_POS}",
                "ヤッホー{EMOJI_POS}{TARGET_NAME}、元気かな{EMOJI_ASK}",
                "{TARGET_NAME}、会社をサボるなんて、悪い子だなぁ{EMOJI_POS}{NANCHATTE}",
                "おはよー！チュッ{EMOJI_POS}",
                "{TARGET_NAME}、久しぶり{EMOJI_POS}",
                "あれ{EMOJI_NEG}{TARGET_NAME}、朝と夜間違えたのかな{EMOJI_ASK}{FIRST_PERSON}はまだ起きてますよ〜{EMOJI_POS}",
                "{TARGET_NAME}、そっちも{WEATHER}なのかな{EMOJI_ASK}",
                "{TARGET_NAME}、こんな遅い時間{EMOJI_NEUT}に何をしているのかな{EMOJI_ASK}",
            ];
        }
        OjisanEmotion::QUESTION => {
            messages = vec![
                "今日はどんな一日だった{EMOJI_ASK}",
                "今日は{WEATHER}だけどなにするのかな{EMOJI_ASK}",
                "{RESTAURANT}好きかな{EMOJI_ASK}",
                "{TARGET_NAME}も今日も2時までお仕事かな{EMOJI_ASK}",
                "ちょっと電話できるかな{EMOJI_ASK}",
                "{DAY_OF_WEEK}曜日は仕事〜{EMOJI_ASK}",
                "今日はもう寝ちゃったのかな{EMOJI_NEUT}",
                "たまには{FIRST_PERSON}にも連絡ほしいな{EMOJI_POS}",
                "{FIRST_PERSON}明日も仕事だけどなかなか寝れないよ〜{EMOJI_NEG}早く{TARGET_NAME}に会いたいよ{EMOJI_NEG}{NANCHATTE}",
                "{TARGET_NAME}と一緒に今度ランチ、したいなぁ{EMOJI_POS}",
                "{TARGET_NAME}と今度イチャイチャ、したいなぁ{EMOJI_POS}",
            ]
        }
        OjisanEmotion::REPORTING =>  {
            messages = vec![
                "今日は{LOCATION}28度だよ{EMOJI_NEG}暑いよ{EMOJI_NEG}ヤケドしないように気をつけないとね{EMOJI_POS}",
                "今日は{LOCATION}30度超えるんだって{EMOJI_NEG}暑いね〜{EMOJI_NEG}こんな日は{FIRST_PERSON}と裸のお付き合い{EMOJI_POS}しよ{EMOJI_POS}{NANCHATTE}",
                "{FIRST_PERSON}はさっきお風呂入ったよ{EMOJI_POS}{TARGET_NAME}とお風呂いきたいなー{EMOJI_POS}{NANCHATTE}",
                "{FIRST_PERSON}は、近所に新しくできた{RESTAURANT}に行ってきたよ。味はまぁまぁだったかな{EMOJI_POS}",
                "そういえば、昨日は例の{RESTAURANT}に行ってきたよ。結構いい雰囲気だったから、オススメだよ{EMOJI_POS}",
                "{FIRST_PERSON}は今日から{LOCATION}へ〜{EMOJI_POS}",
                "お弁当の{FOOD}が美味しくて、それと一緒に{TARGET_NAME}のことも食べちゃいたいな〜{EMOJI_POS}{NANCHATTE}",
                "本日のランチ🍴は奮発して{FOOD}付き{EMOJI_POS}誰だメタボなんて言ったやつは{EMOJI_NEG}",
                "出張で{LOCATION}に行ってきたよ{EMOJI_POS}観光でも、行きたいなぁ{EMOJI_POS}モチロン、{TARGET_NAME}とね",
            ]
        }
        OjisanEmotion::CHEERING => {
            messages = vec![
                "今日も頑張ってね{EMOJI_POS}",
                "{TARGET_NAME}にとって素敵な1日になりますように{EMOJI_POS}",
                "今日は楽しい時間をありがとうね{EMOJI_POS}すごく、楽しかったよ{EMOJI_POS}",
                "早く会いたいな{EMOJI_POS}",
            ]
        }
        OjisanEmotion::INVITATION => {
            messages = vec![
                "今週の{DAY_OF_WEEK}曜日、仕事が早く終わりそうなんだけど、ご飯でもどうかな{EMOJI_ASK}",
                "突然だけど、{TARGET_NAME}は{RESTAURANT}好きカナ{EMOJI_ASK}{DAY_OF_WEEK}曜日ご飯行こうよ{EMOJI_POS}",
                "そろそろご飯行こうよ{EMOJI_POS}ご要望とかはあるのかな{EMOJI_POS}{EMOJI_ASK}",
                "{DAY_OF_WEEK}曜日、会社がお休みになったよ{EMOJI_POS}{TARGET_NAME}は都合どうかな{EMOJI_ASK}{DATE}どう{EMOJI_POS}{NANCHATTE}",
                "天気悪いと気分もよくないよね{EMOJI_NEG}じゃあ今日は会社休んで{FIRST_PERSON}と{DATE}しよう{EMOJI_POS}{NANCHATTE}",
                "今日は天気が悪いね{EMOJI_NEG}こんな日は会社休んで{FIRST_PERSON}と{HOTEL}に行こうよ{EMOJI_POS}{NANCHATTE}",
                "この{HOTEL}、すごいキレイ{EMOJI_POS}なんだって{EMOJI_POS}{FIRST_PERSON}と一緒に行こうよ{EMOJI_POS}{NANCHATTE}",
                "この{HOTEL}、{FOOD}がオイシイんだって{EMOJI_POS}{FIRST_PERSON}と一緒に行こうよ{EMOJI_POS}{NANCHATTE}",
                "住所教えてくれたら、{FIRST_PERSON}が{FOOD}を作ってあげるし、風邪なんてすぐ治る{EMOJI_POS}",
                "{FIRST_PERSON}はプライベートで、{TARGET_NAME}を癒やして{EMOJI_POS}あげたい{EMOJI_POS}って思ってるよ{EMOJI_NEUT}",
            ]
        }
        OjisanEmotion::PRAISING => {
            messages = vec![
                "{TARGET_NAME}、愛しいなぁもう{EMOJI_POS}",
                "{TARGET_NAME}は、スタイルがいいね{EMOJI_POS}",
                "{TARGET_NAME}のお目々、キラキラ{EMOJI_POS}してるね{EMOJI_POS}",
                "{TARGET_NAME}は、お肌がきれい✨だね{EMOJI_POS}",
                "{TARGET_NAME}、髪の毛、切ったのかな{EMOJI_ASK}似合いすぎだよ{EMOJI_POS}",
                "{TARGET_NAME}、可愛らしいね{EMOJI_POS}",
            ]
        }
        OjisanEmotion::ADMIRATION => {
            messages = vec![
                "今から寝ようと思ってたのに、目が覚めちゃったよ{EMOJI_POS}どうしてくれるんだ{EMOJI_POS}",
                "可愛すぎて{FIRST_PERSON}お仕事に集中できなくなっちゃいそうだよ{EMOJI_NEG}どうしてくれるんだ{EMOJI_POS}",
                "ホント可愛すぎだよ〜{EMOJI_POS}マッタクもう{EMOJI_POS}",
                "こんなに可愛く{EMOJI_POS}なっちゃったら{METAPHOR}みたいで{FIRST_PERSON}困っちゃうよ{EMOJI_NEG}",
                "{FIRST_PERSON}、本当に{METAPHOR}かと思っちゃったよ{EMOJI_POS}",
            ]
        }
        OjisanEmotion::SYMPATHY => {
            messages = vec![
                "{TARGET_NAME}{EMOJI_POS}元気、ないのかなぁ{EMOJI_NEG}大丈夫{EMOJI_ASK}",
                "僕は、すごく心配だよ{EMOJI_NEG}そんなときは、美味しいもの食べて、元気出さなきゃだね{EMOJI_POS}",
                "今日も大変だったんだね{EMOJI_NEG}",
                "{FIRST_PERSON}は{TARGET_NAME}の味方だからね{EMOJI_POS}",
                "今日はよく休んでね{EMOJI_NEUT}",
                "くれぐれも体調に気をつけて{EMOJI_NEUT}",
                "{FIRST_PERSON}は{TARGET_NAME}一筋だよ{EMOJI_NEUT}",
                "よく頑張ったね{EMOJI_POS}えらいえらい{EMOJI_POS}",
                "風邪ひかないようにね{EMOJI_POS}",
                "寒いけど、頑張ってね{EMOJI_NEUT}",
                "ゆっくり、身体休めてね{EMOJI_POS}オヤスミナサイ{EMOJI_NEUT}",
            ]
        }
    }
    let mut rng = rand::rng();
    messages.shuffle(&mut rng);
    messages[0]
}
