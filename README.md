# dyntrait-test

## これは何

Rust の `dyn Trait` を使っていろいろするリポジトリ

## ファイル構成

Cargo workspace で複数の crate を含んでいます

- app(bin)
  - 依存: serde_json, base, ext
  - `Box<dyn Effect>` のシリアライズ・デシリアライズを試すコード
  - `cargo run` で実行できる
- base(lib)
  - 依存: serde, typetag
  - `Object` 構造体が定義されている
  - `Effect` トレイトが定義されている
  - `Effect` トレイトを実装した `ForAllEffect` 構造体が定義されている
- ext(lib)
  - 依存: serde, typetag, base
  - `Effect` トレイトを実装した `SpecificEffect` 構造体が定義されている
- test1(lib)
  - 依存: serde_json, base
  - `ForAllEffect` について `Box<dyn Effect>` としてデシリアライズのテスト
- test2(lib)
  - 依存: serde_json, base, ext
  - `ForAllEffect, SpecificEffect` について `Box<dyn Effect>` としてデシリアライズのテスト
  - `ext::void()` による `ext` のコンパイル強制
- test3(lib)
  - 依存: serde_json, base, ext, dyntrait_de_wrapper
  - `ForAllEffect, SpecificEffect` について `Box<dyn Effect>` としてデシリアライズのテスト
  - `dyntrait_de_wrapper::define_de_dyntrait_json!` によるデシリアライズ候補リストの明示

## 依存クレート

- serde
  - https://github.com/serde-rs/serde
  - シリアライズ・デシリアライズフレームワーク
  - `Serialize, Deserialize` derive マクロによってシリアライズ・デシリアライズ可能な構造体に拡張する
- serde_json
  - https://github.com/serde-rs/json
  - serde の Json 形式用クレート
  - `serde_json::to_string` や `serde_json::from_str` を使う
- typetag
  - https://github.com/dtolnay/typetag
  - `typetag::serde` proc_macro などを提供する
  - serde と合わせることで `&dyn Trait` のシリアライズ、`Box<dyn Trait>` へのデシリアライズが可能になる
- dyntrait_de_wrapper
  - https://github.com/n-ari/dyntrait-de-wrapper
  - `typetag` によってデシリアライズ可能にした `Trait` とそれを実装した型について、デシリアライズ先の型のリストを明記してデシリアライズ関数を定義するマクロを提供する
  - `typetag` と複数クレートにまたがったトレイト実装を使った場合、デシリアライズ先の型の含まれたクレートのコンパイルを強制しないといけない問題も解決する
  - `typetag` が実装する `typetag_name` 関数が `&self` を要求するため、現状は `Trait` を実装した各型に `Default` トレイトが実装されていることを要求する

