// テーマ:
// スプレッドシート(表計算ソフト)のデータを構造化してみよう。
// そのデータと、データへの操作をそれぞれ考えていこう。

fn main() {}

// -----------------------------
// |   | A | B | C | D | E | F |
// -----------------------------
// | 1 |  1|  2| 3 |   |   |hi |
// -----------------------------
// | 2 |   |   |   |   |foo|   |
// -----------------------------
// | 3 |  3|  1| 4 |   |   |   |
// -----------------------------
// | 4 |   |   |   |sum|  =|  7|
// -----------------------------

// 注意: 以下は設計手順のひとつの例であり、どこから手を付けるか、といったことはそれぞれの工夫次第である。
// 課題(VERY HARD): 以下の手順を見ずに自身でどのような設計手順で進めればよいか考察せよ。
//      VERY HARDとしたが、まずトライしてみることを推奨する。

// ... 以下ネタバレ防止の空白
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
// 課題(MEDIUM):
//      セルデータが空、文字列、数値だけである場合の表計算データの構造体を考えよ。
//      無限の広さに拡張できる前提で設計せよ。
//      どのような方法が考えられるだろうか。
//      extra: 複数の方法を考え、それぞれの表現方法のメリットデメリットを考えよ。
//      extra: 数値セルとひとことに言うが、どのような選択肢があるだろうか。
// 課題(MEDIUM): もうひとつ自身でセルデータのタイプを追加してみよ。
// 課題(MEDIUM): データの取得メソッドを実装せよ。
// 課題A(MEDIUM):
//      データの文字列としての取得メソッドを実装せよ。
//      セルの幅が決まっている状態でフォーマットせねばならないわけなので、widthを受けとって、常にその長さの文字列を変えすようにしてみよ。
//      extra: HARD: アスキー文字だけに限定し、 `struct AsciiChar(char)` などを作成してその列 `struct AsciiString(Vec<AsciiChar>)` を利用することを考えてみよ。lv1_string_specializeも参照せよ。
// 課題B(MEDIUM): データの書き換えメソッドを実装せよ。
// 課題(MEDIUM):
//      表計算データのセルのアライン(左、中央、右揃えなど)を実現せよ。
//      また、アラインの設定関数を実装せよ。
//      課題Aの結果がこの設定を反映するようにせよ。
// 課題(HARD):
//      表計算データの数値セルの精度指定(小数点以下第n位まで表示)を実現せよ。
//      数値精度の設定関数を実装せよ。
// 課題(HARD): 表計算データの数値セルの最大値、最小値の設定を実現せよ。
// 課題(HARD):
//      上記の見た目でascii出力する std::fmt::Format トレイトを実装せよ。
//      HARDとしているが、デバッグには重要なので、まずは簡単な見た目でトライしてみるのがよいだろう。
//      ヒント: 先に Format をセルデータタイプのほうに実装するのがよいだろう。
// 課題(HARD):
//      表計算データでみかけることのある、列、行そのものを対象に書式指定できる機能を実装せよ。
//      現在見えている範囲の外にデータが入っても適切に反映されるように設計せよ。
//      ここで、セル自体の書式設定とも共存できるような方法で設計を考えてみよ。
//      extra: 他にどのような実現方法が考えられるだろうか。それぞれのメリットデメリットはなんだろうか。
// 課題(HARD): 計算セルを考案せよ。ただし、簡単のために以下の計算式のみができるだけでよいだろう。
//      - 特定の指定された2つのセルの和
//      - 特定の列、または行全体の合計値
//      extra: 他に、簡単なもので良いので計算式タイプを追加してみよ。
//      課題Aのメソッドは計算結果を返すべきだろう。
//      指定先のセルが空の場合はどう扱うべきだろうか。
//      その他に考慮すべきコーナーケースがないだろうか。
// 課題C(MEDIUM):
//      特定のセルから別のセルにコピーする関数を実装せよ。
// 課題(HARD):
//      編集系のセルの選択をより一般化してみよ。矩形選択や、離れ離れになってるセルを選択してコピペする場合、行や列の選択はどうだろうか。
//      課題B,Cを実施している場合は、それらをこの課題の方式に変更せよ。
// 課題D(HARD):
//      表計算にある、引き伸ばしてコピーする機能を実装せよ。シグニチャをどうすればよいか工夫して考えよ。
//      文字 `["A"、"B"]` などを選択して引き伸ばす場合は `["A", "B", "C", "D" …]` となるようにしてみよ。
//      extra: 他の引き伸ばしパターンを考えて実現せよ。
//      extra: いろいろな引き伸ばしパターンがあると個別にON/OFFしたくなるかもしれない。どのように実現できるだろうか。
//      extra: VERY HARD:
//          外部からアドオンとしてパターンを取り込んで利用したくなるかもしれない。どうすれば実現できるだろうか。
// 課題(HARD):
//      上記の計算セルのうち、単一セルを参照するものについて、絶対参照と相対参照を区別するようにせよ。。
//      これは通常の表計算ソフトで `A1` などと対比して `A$1$` などと書かれるものである。
//      課題C,D を実装してる場合は、それらのメソッドがこれを考慮するように変更せよ。
// 課題(HARD): その他、自身で機能を考え実装せよ。
// 課題(VERY HARD): これまで実装してきた中で、自作トレイトを利用するのが適切な箇所を探し、実践せよ。
