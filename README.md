# Tamanegi-Error

![main branch parameter](https://github.com/tnishinaga/tamanegi-error/actions/workflows/main.yml/badge.svg?branch=main)

Tamanegi-Errorは、Rustの **“伝搬元のエラーや伝搬過程のコンテクスト(location等)”** を保持することで伝搬過程の追えるエラーの設計およびログ出力をサポートするライブラリです。
Rustではエラーの伝搬過程で元のエラーのコンテクストが失われてしまうことがありますが、SnafuとTamanegi-Errorを使ってエラーを設計することで、大元となったエラーとその発生場所や伝搬過程をさかのぼって確認できます。

このライブラリは[Stack Error](https://greptime.com/blogs/2024-05-07-error-rust#how-error-looks-like-with-virtual-user-stack)のアイデアを元に作られました。詳しくは [StackErrorとの関係](#stackerrorとの関係) をご確認ください。


## 特長

- **エラー伝搬過程の保存** の手助け
    - 伝搬されるエラーを単に置き換えるのではなく、元のエラーとコンテクストを内包することで、最終的なエラーから発生元までをたどれるようにできます。これはSnafuの機能に依存します。
    - Tamanegi-Errorはlocationコンテクストを必須とし、伝搬位置の保存を確実にします。
- **伝搬過程のログ出力機能追加**
    - Tamanegi−Errorを実装したエラー型に対し、エラーの伝搬過程を追ってログに出力する機能を追加します
- **no_std対応**
    - backtrace等を利用しづらいno_std環境でもエラーの伝搬過程を追えます

## 提供する機能

- RAM使用量を抑えたlocation型
- locationコンテクスト付与の強制
- Errorの来歴を表示するDebug traitの実装


## 利用方法

以降、ライブラリの利用方法について紹介します。

### ライブラリの追加方法

Cargoを利用して本ライブラリをプロジェクトに追加します。
このライブラリは[snafu](https://crates.io/crates/snafu)に依存しているため、snafuの追加も必要となります。

```bash
cargo add tamanegi-error
cargo add snafu
```

### 利用例

以下のようにSnafuを実装したError型にTamanegiErrorを追加して利用します。

> [!NOTE]
> **現状の**[^why_dont_support_thiserror]Tamanegi-Errorはsnafuに強く依存しており、Snafuを用いて作られたError型に対し、以下の機能を追加するderive macroを提供します。
> 言い換えれば、Tamanegi−Errorを使うには、最終的なエラー(leaf)を除いて伝搬過程を追いたいエラー型は、すべてsnafuを用いて作られている必要があります。

[^why_dont_support_thiserror]: Tamanegi-Errorはエラーに対しlocation以外にもさまざまなコンテクストを追加していく設計を想定しています。一方、thiserrorの `#[from]` macroは型の要素としてsourceまたはbacktraceしか受け付けてくれないという課題があり、そのままの採用が難しいとなりました。

TamanegiErrorを実装するError型にはlocationメンバーが必須となります。

```rust
use snafu::{GenerateImplicitData, Snafu};
use tamanegi_error::{TamanegiError, location::StaticLocationRef};

#[derive(Snafu, TamanegiError)]
pub struct ErrorSubA {
    #[snafu(implicit)]
    location: StaticLocationRef,
}

impl ErrorSubA {
    pub fn new() -> Self {
        Self {
            location: StaticLocationRef::generate(),
        }
    }
}

#[derive(Snafu, TamanegiError)]
#[snafu(context(false))]
struct MyError {
    #[snafu(source)]
    source: ErrorSubA,
    #[snafu(implicit)]
    location: StaticLocationRef,
}

fn err() -> Result<(), MyError> {
    let err: Result<(), ErrorSubA> = Err(ErrorSubA::new());
    let _ = err?;
    Ok(())
}

fn main() {
    if let Err(e) = err() {
        println!("{:?}", e);
    }
}
```

上記の例のように、最初に発生したErrorを含めて伝搬過程のすべてのErrorがTamanegiErrorを実装している場合、実行結果は以下のようになります。

```
kagamimochi-error ❯❯❯ cargo run --example basic_struct

1: MyError, at examples/basic_struct.rs:29:13
0: ErrorSubA, at examples/basic_struct.rs:13:23
```

伝搬過程にTamanegiErrorを実装していないErrorがある場合、そのErrorのdebugを表示して終了します。


## StackErrorとの関係

「エラーを入れ子構造にし、さらにlocationを追加して伝搬過程を追えるようにする」というアイデアは[Stack Error](https://greptime.com/blogs/2024-05-07-error-rust#how-error-looks-like-with-virtual-user-stack)を素にしています。
このStackErrorにはbacktraceと比べて、エラーの伝搬が追えることと軽量であるというメリットがありました。
しかし、このStack Errorはstdやallocに依存しているため、そのままno_std環境に流用できませんでした。
そのため、StackErrorのアイデアのみを素にしてno_stdでも動作するよう1から書き直した結果、Snafuの機能とproc macroを使ってStackErrorに似た仕組みが完成しました。
このなかのproc macro部分を抜き出してライブラリ化したのがTamanegi-Errorライブラリです。


## Tamanegi Error命名の由来

エラーを入れ子にする構造から玉ねぎ(Onion)を連想しましたが、作者は日本人なのでOnionの日本語であるTamanegiを採用することにしました。

## ライセンス

本プロジェクトは MIT License OR Apache 2.0 License のもとで公開されています。詳細については LICENSE ファイルを参照してください。

## 作者

- Toshifumi Nishinaga
    - GitHub: @tnishinaga