// テーマ:
// 曜日を表現する Weekday 構造体を作ります。
// Weekday::Sun で日曜日を表す、のようにしてください。
// 一週間の英語略語: Sun, Mon, Tue, Wed, Thu, Fri, Sat

// 各課題について、 #[test] からコメントアウトして実装をせよ。
// テストはぜひとも自身でさらにケースを追加せよ。

/// 曜日
#[derive(Debug, PartialEq, Eq)]
enum Weekday {
    Todo,
}
// 補足:
// Debug は `println!("{:?}", w);` 等でデバッグ表示することを可能にする。
// PartialEq,Eq は `a == b` や `assert_eq!(a, b);` で比較することを可能にする。

// 課題(EASY): 曜日を日本語にするメソッド to_ja を実装せよ。
// #[test]
// fn test_weekday_to_ja() {
//     assert_eq!(Weekday::Sun.to_ja(), "日曜日");
//     assert_eq!(Weekday::Tue.to_ja(), "火曜日");
// }

// 課題(EASY): 曜日を指定された言語にするメソッド format_in を実装せよ。
// 言語の表現方法を工夫せよ。
// #[test]
// fn test_weekday_format_in_1() {
//     assert_eq!(Weekday::Sun.format_in_1(/* ja */, "日曜日");
//     assert_eq!(Weekday::Sun.format_in_1(/* en */, "Sun");
//
//     assert_eq!(Weekday::Tue.format_in_1(/* ja */, "火曜日");
//     assert_eq!(Weekday::Tue.format_in_1(/* en */, "Tue");
// }

// 課題(EASY-MEDIUM): 同一の言語でも複数のスタイルがありうる場合の表現方法を考察せよ。
// EASY: 言語・スタイルについて2種類の構造化を考えよ。
// MEDIUM: 言語・スタイルについて3種類の構造化を考えよ。
// #[test]
// fn test_weekday_format_in_2() {
//     assert_eq!(Weekday::Sun.format_in_2(/* ja short */, "日");
//     assert_eq!(Weekday::Sun.format_in_2(/* ja long */, "日曜日");
//
//     assert_eq!(Weekday::Sun.format_in_2(/* en short */, "Sun");
//     assert_eq!(Weekday::Sun.format_in_2(/* en long */, "Sunday");
// }

// 課題(EASY): ある曜日から、その日を除いて曜日xになるのにかかる最小の日数を計算するメソッドを実装せよ
// extra(MEDIUM): 戻り値の整数値タイプはどのような根拠のもとで、どのように選択すると良いだろうか。
// #[test]
// fn test_weekday_how_many_days_until_next() {
//     assert_eq!(Weekday::Sun.how_many_days_until_next(Weekday::Sun, 7);
//     assert_eq!(Weekday::Sun.how_many_days_until_next(Weekday::Mon, 1);
//     assert_eq!(Weekday::Sun.how_many_days_until_next(Weekday::Tue, 2);
//     assert_eq!(Weekday::Sun.how_many_days_until_next(Weekday::Sat, 6);
//
//     assert_eq!(Weekday::Sat.how_many_days_until_next(Weekday::Sun, 6);
//     assert_eq!(Weekday::Sat.how_many_days_until_next(Weekday::Mon, 7);
//     assert_eq!(Weekday::Sat.how_many_days_until_next(Weekday::Tue, 1);
//     assert_eq!(Weekday::Sat.how_many_days_until_next(Weekday::Sat, 5);
// }

// 課題(HARD): 曜日a,bについて、for x in a..bで曜日aからbの前日までを回せるようにせよ。
// ヒント:
// - 一般に x,y:T について x..y のタイプは Range<T> である
//   - https://doc.rust-lang.org/std/ops/struct.Range.html
// - 一般に for _ in iterable {} の iterable に来ることができるのは iter::IntoIterator
// を実装しているタイプである
//   - https://doc.rust-lang.org/std/iter/trait.IntoIterator.html を参照せよ
// #[test]
// fn test_weekday_range() {
//     let mut v = vec![];
//     for x in Weekday::Sun..Weekday::Sun {
//         v.push(x);
//     }
//     assert_eq!(
//         v,
//         vec![
//             Weekday::Sun,
//             Weekday::Mon,
//             Weekday::Tue,
//             Weekday::Wed,
//             Weekday::Thu,
//             Weekday::Fri,
//             Weekday::Sat
//         ],
//     );
//
//     assert_eq!(
//         (Weekday::Thu..Weekday::Mon).collect::<Vec<_>>(),
//         vec![Weekday::Thu, Weekday::Fri, Weekday::Sat, Weekday::Sun],
//     );
// }

// 課題(HARD): a..b が実現できれば、他に類似で欲しいものはないだろうか。

// 課題(EASY): 日付から曜日を計算する関数を実装せよ。
// 日付の表現は工夫して考えよ。カレンダーは当然自由だが、グレゴリオ歴について仕様を調べると良いだろう。
// テストケースも自身で十分に実装せよ。コーナーケースは十分に考慮できているだろうか？
// #[test]
// fn test_weekday_of_date() {
//     assert_eq!(Weekday::of_date(/* date */), Weekday::Fri);
// }

// 課題(EASY): その曜日からn日後(負なら-n日前)の曜日を計算するメソッドを実装せよ。
// #[test]
// fn test_weekday_after() {
//     assert_eq!(Weekday::Sun.after(0_isize), Weekday::Sun);
//     assert_eq!(Weekday::Sun.after(1), Weekday::Mon);
//     assert_eq!(Weekday::Sun.after(2), Weekday::Tue);
//     assert_eq!(Weekday::Sun.after(3), Weekday::Wed);
//     assert_eq!(Weekday::Sun.after(100), Weekday::Tue);
//     assert_eq!(Weekday::Sun.after(10000000000000000), Weekday::Thu);
//
//     assert_eq!(Weekday::Sun.after(-1), Weekday::Sat);
//     assert_eq!(Weekday::Sun.after(-2), Weekday::Fri);
//     assert_eq!(Weekday::Sun.after(-3), Weekday::Thu);
//     assert_eq!(Weekday::Sun.after(-100), Weekday::Fri);
// }

// 課題(EASY): afterがあれば、もうひとつ欲しくならないだろうか。
// テストも自身で実装せよ。
// #[test]
// fn test_weekday_???() {
// }

// 課題(VERY HARD): Weekday::afterを利用して、なにかしらの演算子を実装せよ。
// 実装可能な演算子は https://doc.rust-lang.org/std/ops/index.html を参照せよ。
// #[test]
// fn test_weekday_ops_???() {
//     assert_eq!(Weekday::Sun ?????, ?????);
// }

// 課題(MEDIUM): FromStr を実装せよ。
// 自身でどのような文字列を曜日としてパースできるとよいか考察せよ。
// また、パースできない文字列というのはあるだろうか。どうすればよいだろうか。
//
// FromStrの実装方法は以下を参照。
// https://doc.rust-lang.org/stable/std/str/trait.FromStr.html
// #[test]
// fn test_weekday_parse() {
//     assert_eq!("???".parse::<Weekday>(), Ok(Weekday::Sun));
//     assert_eq!("???".parse::<Weekday>(), Ok(Weekday::Mon));
//     assert_eq!("???".parse::<Weekday>(), Ok(Weekday::Tue));
//
//     assert_eq!("random string".parse::<Weekday>() /* ??? */,);
// }

// 課題(MEDIUM): 比較 `a < b` も PartialOrd/Ord を介してユーザー定義が可能だ。
// https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
// Weekday に比較を実装することはできるだろうか。それは妥当な設計と言えるだろうか。

// 課題(MEDIUM):
// 自身でさらに、このWeekdayをライブラリとして提供するうえであったほうがよいと思える関数を考え、設計・実装せよ。
// 簡単なものでよい。
// #[test]
// fn test_weekday_???() {
// }

fn main() {
    // dbg!(Weekday::Sun);
}
