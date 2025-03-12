# Color Code Parser

### 概要
`Color Code Parser` は、16進カラーコード（例: `color#RRGGBB` または `color#RRGGBBAA`）を解析する Rust プロジェクトです。「color」の接頭辞を省略した形式もサポートしています。

このプロジェクトでは [nom](https://github.com/Geal/nom) を使用して文字列を効率的にパースし、カラーコードを構造体として扱えるようにします。

---

### 実行方法
以下の手順でプロジェクトをビルドおよび実行してください。

1. **依存関係インストール**  
   必要なクレート（今回は `nom`）を `Cargo.toml` ファイルに設定してあります。`cargo build` または `cargo run` を実行するだけです。

2. **プロジェクトの実行**
   ```bash
   cargo run
   ```
   サンプル入力文字列として `color#2F14DF11` が渡され、結果が出力されます。

3. **テストの実行** (動作確認)  
   プロジェクト内の単体テストを実行するには以下を利用してください。
   ```bash
   cargo test
   ```

---

### 使用例

- 入力: `color#2F14DF11`  
  出力:
  ```
  Parsed color: Color { red: 47, green: 20, blue: 223, alpha: Some(17) }
  Remaining input: ""
  ```

- 入力: `#2F14DF`  
  出力:
  ```
  Parsed color: Color { red: 47, green: 20, blue: 223, alpha: None }
  Remaining input: ""
  ```

---

### 実装の概要

- **`ColorCode` 構造体**  
  `ColorCode` は `Parser` トレイトを実装した構造体で、入力文字列の解析を管理します。

- **`Color` 構造体**  
  パース結果を格納するためのデータ構造で、以下のフィールドを持ちます。
    - `red: u8`
    - `green: u8`
    - `blue: u8`
    - `alpha: Option<u8>`

- **主要な関数**
    - `hex_color(input: &str)`  
      文字列の解析を行い、`Color` 構造体を返します。
    - `hex_primary(input: &str)`  
      赤、緑、青（RGB）の各値を16進数から読み取る補助関数です。
    - `hex_rgba(input: &str)`  
      `RRGGBBAA` の形式でカラーをパースし、`Color`構造体の `alpha` に値を格納します。

---

### テストケース

一部のテストケース例を以下に示します。

1. 正常なカラーコード解析
   ```rust
   assert_eq!(
       hex_color("color#2F14DF"),
       Ok((
           "",
           Color {
               red: 47,
               green: 20,
               blue: 223,
               alpha: None,
           }
       ))
   );
   ```

2. 設定ミスのチェック（データ不正を認識）
   ```rust
   assert!(hex_color("color#2F14DG").is_err());
   ```

3. "color"プレフィックス無し
   ```rust
   assert_eq!(
       hex_color("#2F14DF"),
       Ok((
           "",
           Color {
               red: 47,
               green: 20,
               blue: 223,
               alpha: None,
           }
       ))
   );
   ```

4. アルファチャンネル付きの解析
   ```rust
   assert_eq!(
       hex_color("color#2F14DF04"),
       Ok((
           "",
           Color {
               red: 47,
               green: 20,
               blue: 223,
               alpha: Some(4),
           }
       ))
   );
   ```

---

### 今後の予定
以下の機能などを追求する予定があります：
- 入力と形式のバリデーションを拡張
- Webベースツール（カラー表示 GUI）との統合
- エラーメッセージをより詳細にする

---

### ライセンス

**Color Code Parser** は MIT License のもとで提供されています。詳細は [LICENSE](../LICENSE) ファイルをご覧ください。