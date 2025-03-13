# Dice Roll Parser

### 概要
`Dice Roll Parser` は、ダイスロールを表す文字列（例: `2D6+4` や `5D10`）を解析して計算に使用可能な構造体の形式に変換する Rust プロジェクトです。このプロジェクトでは [nom](https://github.com/Geal/nom) を使用して効率的に文字列をパースします。

正の修正値(`+`)、負の修正値(`-`)、および修正値なしのダイスロール表記に対応しています。

---

### 実行方法

1. **依存関係のインストール**
   プロジェクトで使用されている `nom` クレートは、すでに `Cargo.toml` に設定されています。以下を実行することでプロジェクトのビルドを行えます。
   ```bash
   cargo build
   ```

2. **プロジェクトの実行**
   プロジェクトに含まれているサンプルコードを実行することで、入力されたダイス文字列を解析できます。
   ```bash
   cargo run
   ```
   例えば、入力が `2D6+` の場合、解析された結果が出力されます。

3. **テストの実行**
   パーサーの動作確認として用意した一連のテストを実行するには、次のコマンドを利用してください。
   ```bash
   cargo test
   ```

---

### 使用例

- **入力:** `2D6+4`  
  **出力:**
  ```
  Input: 2D6+4
  Remaining: 
  Result: DiceRoll { sides: 6, count: 2, modifier: 4 }
  ```

- **入力:** `5D10-4`  
  **出力:**
  ```
  Input: 5D10-4
  Remaining: 
  Result: DiceRoll { sides: 10, count: 5, modifier: -4 }
  ```

- **入力:** `5D10`  
  **出力:**
  ```
  Input: 5D10
  Remaining: 
  Result: DiceRoll { sides: 10, count: 5, modifier: 0 }
  ```

---

### 実装の概要

#### **DiceRoll 構造体**
入力文字列をパースして得られたダイスロールの情報を格納するデータ構造です。以下のフィールドを持ちます：
- `sides: u32`  
  ダイスの面数（例: 6）。
- `count: u32`  
  振るダイスの個数（例: 2）。
- `modifier: i32`  
  修正値（例: +4, -4, または 0）。

#### **BasicRoll 構造体**
この構造体はカスタムの `Parser` トレイトを実装しており、入力文字列から `DiceRoll` データを生成します。以下の流れに従って文字列を解析します：
1. ダイスの個数（`count`）を取得。
2. 'D' の存在を確認。
3. ダイスの面数（`sides`）を取得。
4. 修正値（`modifier`）の有無を確認し、処理。

#### **主要な関数**
- `parse_modifier`  
  ダイス文字列の修正値部分（例: `+4` や `-4`）を解析します。この関数では、修正値が存在しない場合は `0` を返します。

---

### テストケース

1. **修正値が正の場合のテスト**
   ```rust
   let mut basic_parser = BasicRoll {};
   assert_eq!(
       basic_parser.parse("2D6+4").unwrap().1,
       DiceRoll { sides: 6, count: 2, modifier: 4 }
   );
   ```

2. **修正値が負の場合のテスト**
   ```rust
   let mut basic_parser = BasicRoll {};
   assert_eq!(
       basic_parser.parse("5D10-4").unwrap().1,
       DiceRoll { sides: 10, count: 5, modifier: -4 }
   );
   ```

3. **修正値がない場合のテスト**
   ```rust
   let mut basic_parser = BasicRoll {};
   assert_eq!(
       basic_parser.parse("5D10").unwrap().1,
       DiceRoll { sides: 10, count: 5, modifier: 0 }
   );
   ```

4. **修正値が「符号のみ」の場合のテスト**
   ```rust
   let mut basic_parser = BasicRoll {};
   assert!(basic_parser.parse("5D10+").is_ok());
   ```

---

### 今後の予定

- ダイスロール結果を直接計算するオプションの追加。
- 修正値がより複雑な数式の場合に対応するパーサーの拡張。
- パフォーマンスの最適化。

---

### ライセンス

**Dice Roll Parser** は MIT License のもとで提供されています。詳細については [LICENSE](../LICENSE) ファイルをご確認ください。