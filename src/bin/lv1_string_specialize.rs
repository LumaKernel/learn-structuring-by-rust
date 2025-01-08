// テーマ:
// データベースにdescription(説明文)という文字列カラムを作ったとしましょう。
// その説明文は未設定の場合は、特別な処理(例えば灰色に `[説明なし]`
// と表示したい、など)をしたいとします。

// ここでは、あくまでもWebフロントエンドのようなものがあって、以下でそれを実現できると仮定します。
fn render_black(s: &str) -> String {
    format!("<text color=black>{}</text>", s.replace("<", "&lt;"))
}
fn render_gray(s: &str) -> String {
    format!("<text color=gray>{}</text>", s.replace("<", "&lt;"))
}

// このテーマのテストプレフィックスは `test_ss_` としています。

// 課題(EASY): まずテーマにあるデータの表現をStringで行うことを考えてください。
// render_recordを実装してみてください。
#[test]
fn test_ss_use_string() {
    #[derive(Debug)]
    struct Record {
        description: String,
    }

    fn render_record(record: Record) -> String {
        todo!()
    }

    assert_eq!(
        render_record(Record {
            description: "Hello".to_string()
        }),
        "<text color=black>Hello</text>",
    );
    assert_eq!(
        render_record(Record {
            description: "".to_string()
        }),
        "<text color=gray>[説明なし]</text>",
    );
}

// 課題(MEDIUM): 上記の方法による欠点を考えてください。
// ヒント:
// - Recordだけを見て .description が未設定とみなせる条件が分かるでしょうか
// - 未設定とみなすべき条件が変わった場合はどうでしょうか。
// - .descriptionはきっと特定の文字しか許さない、といった追加の制約がある場合はどうでしょうか。

// 最初のアプローチとして、NonEmptyStringという空ではない文字列のみを許容する方法を考えてください。
mod non_empty_string {
    pub struct NonEmptyString(String);
    // ノート:
    // 上記の書きかたは `NonEmptyString { v: String }` と等価と考えてください。
    // なお、 `s.v` に相当するものは `s.0` でアクセス可能です。

    // ノート:
    // NonEmptyString(s) とすれば上記の構造体が構築できる。
    // ところで、この構築方法をユーザーができてしまうと、 `NonEmptyString("".to_string())` とされてしまう。
    // なので直接のコンストラクタは公開せず、以下の NonEmptyString::new によって提供する。
    // mod で囲っているのは、モジュールとして分離せねばの直接コンストラクタが利用できてしまうから。

    impl NonEmptyString {
        // 課題(EASY): ::new を実装してください。
        // ヒント: panic!() か assert!() を利用して許容しないパターンではパニックせよ。
        pub fn new(s: &str) -> Self {
            todo!()
        }

        // 課題(HARD): パニックはRustにおけるエラー表現のひとつであるが、これは基本的には
        // 「 パニックを起こさないように使う方法はこうであるから、この条件を満たして実装せよ。
        // (さもなくばプログラムを強制中断する) 」
        // といった意図で使用すべきものである。
        // 空文字列であることの保証を NonEmptyString::new の使用者に求めるのは妥当な範囲であるが、
        // その条件がより複雑になれば妥当でないと言える境界があるだろう。
        // その境界とはどれくらいだろうか、考察せよ。

        // 課題(EASY):
        // パニックによるエラー表現は基本として使用者がパニックを起こさないよう利用しなければならない。
        // エラーが起きるかもしれず、起きた後で使用者がそれを知って分岐する場合、Result
        // を利用すべきである。
        // Resultを返す関数のバリエーションを持つ場合のひとつの慣習は `try_` を接頭することである。
        // このバリエーションを実装せよ。
        pub fn try_new(s: &str) -> Result<Self, String> {
            todo!()
        }

        // 課題(EASY):
        // Rustの慣習として、所有権を受けとって所有権を返す変換のための関数は `into_*`
        // という名称を利用する。
        // extra: なお、その変換のコストは非常に小さいことが期待される。そうでなければ所有権どうでも `to_` を利用することがある。
        // into_string を実装せよ。
        pub fn into_string(self) -> String {
            todo!()
        }

        // 課題(EASY):
        // Rustの慣習として、参照を受け取って参照を返す変換のための関数は `as_*`
        // という名称を利用する。特に、文字列は `as_string` や `as_string_ref` ではなく `as_str` になるので注意。
        // as_str を実装せよ。
        pub fn as_str(&self) -> &str {
            todo!()
        }

        // 課題(VERY HARD):
        // これは設計の課題というよりRustの話だが、借用&strをより理解したい場合はチャレンジせよ。
        // `as_html` として `<p>` と `</p>` で囲って返す関数を&strを返すように設計したいがそれは根本的に不可能、もしくは無意味なものである。
        // なぜだろうか。
        // ヒント: String::trim は &str を返すが String::replace はそうではない。この差はなんだろうか。
        pub fn as_html(&self) -> &str {
            let html = format!("<p>{}</p>", self.0);
            // &html (できない)
            // html.as_str() (できない)
            todo!()
        }

        // 課題(HARD):
        // trim(NonEmptyString): NonEmptyString を実装することを考えよう。
        // 借用のことは忘れて所有権を新たに作って返す方法でかまわない。
        // なお、trimをそのまま実装することは妥当だろうか？
        // 必要に応じて関数名やそのシグニチャを自身で変更せよ。
        pub fn trim(self) -> Self {
            todo!()
        }

        // 課題(MEDIUM):
        // 他に自身でメソッドを考え実装せよ。
        // extra: VERY HARD: ただし、下で登場するDerefを用いる方法をもってしても意義があるものを考えよ。
    }

    // 課題(EASY):
    // Cloneは通常 #[derive(Clone)] をすれば十分だが、これはフィールドすべてを.clone()するように自動で実装してくれているにすぎない。
    // 自身で以下のように実装することも可能である。
    // Cloneを実装せよ。
    impl Clone for NonEmptyString {
        fn clone(&self) -> Self {
            todo!()
        }
    }

    // 課題(MEDIUM-VERY HARD):
    // Clone以外にもビルトインでderiveできるものには以下のようなものがある。
    // これらそれぞれについて自動実装/手動実装することは妥当か？
    // - Default (MEDIUM)
    // - PartialEq, Eq (MEDIUM)
    // - PartialOrd, Ord (MEDIUM)
    // - Debug (HARD)
    // - Hash (HARD)
    //   - ヒント: std::collections::HashMap<Key, Value> は Key: Hash を要求する。HashMap のみの都合で考えてみよ。
    // - Copy (VERY HARD)

    // 課題(EASY):
    // なにかしらのTについて `T` に `into_*` できることを表わす専用のトレイト Into/From がある。
    // ノート: Fromを実装するとIntoも自動で実装される。IntoではなくFromを実装するのが慣習である。
    // From<NonEmptyString> を String に実装せよ。
    impl From<NonEmptyString> for String {
        fn from(s: NonEmptyString) -> Self {
            todo!()
        }
    }

    // 課題(EASY):
    // なにかしらのTについて `&T` に `as_*` できることを表わす専用のトレイト AsRef がある。
    // AsRef<str> を実装せよ。
    impl AsRef<str> for NonEmptyString {
        fn as_ref(&self) -> &str {
            todo!()
        }
    }

    // 課題(EASY):
    // なにかしらのTについて `T` から `try_from_*` できることを表わす専用のトレイト TryFrom
    // がある。
    // ノート: TryFromを実装するとTryIntoも自動で実装される。TryIntoではなくTryFromを実装するのが慣習である。
    // TryFrom<String> を実装せよ。
    impl TryFrom<String> for NonEmptyString {
        type Error = String;

        fn try_from(s: String) -> Result<Self, Self::Error> {
            todo!()
        }
    }

    // 課題(EASY):
    // NonEmptyStringはほぼStringであり、&strに対して行えなる操作は基本できてほしいだろう。
    // しかし、それを行うには、すべての箇所で .as_str() をせねばならない。
    // Derefを利用すればこれを短縮可能である。
    // 例えば文字列としての長さを `s.as_str().len()` の代わりに `s.len()` として直接記述できる。
    // Derefを実装せよ。また、各種&strのメソッドで挙動を確認せよ。
    impl std::ops::Deref for NonEmptyString {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            todo!()
        }
    }
}
#[test]
fn test_ss_use_non_empty_string() {
    use non_empty_string::NonEmptyString;
    #[derive(Debug)]
    struct Record {
        description: (), // タイプを書きかえよ
    }

    fn render_record(record: Record) -> String {
        todo!()
    }

    assert_eq!(
        render_record(Record {
            description: todo!(),
        }),
        "<text color=black>Hello</text>",
    );
    assert_eq!(
        render_record(Record {
            description: todo!(),
        }),
        "<text color=gray>[説明なし]</text>",
    );
}

// 課題(HARD):
// NonEmptyString以外のアプローチを考えよ。たとえばDescriptionStringのようにより特化させるとどうだろうか。
// ヒント: String は Unicode文字列 を表現する構造体である。
// ヌル文字を許可したままにしておく必要があるだろうか。
// 改行文字をCRLF両方許可したままにしておく必要があるだろうか。
// その他の制御文字についてはどうだろうか。
// 空白文字のみの文字列は許可すべきだろうか。
// 先頭、末尾に空白文字がある文字列は許可すべきだろうか。
// (これらの問いは「許可すべきではない」を模範回答としているわけではない。)
// そして、それらをどう表現するべきだろうか。実際に実装してみよ。

fn main() {}
