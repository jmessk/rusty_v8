# Isolate スレッド移動対応の調査・修正まとめ

このドキュメントは、`rusty_v8` フォーク上で実施した **「Isolate を別スレッドへ移動し、再度 JS を実行する」** ための調査と修正内容の要約です。  
元プロジェクトへ移植する際のチェックリストとして使える形にまとめています。

## 目的

- Isolate をあるスレッドで作成し、別スレッドへ移動後に再度 `Script::compile/run` を成功させる
- `Global<Context>` を保持したままの往復（main ↔ worker）でも実行できることを確認する
- 必要なら `unsafe` を許容して、まずは動作可能な最小経路を確立する

## 調査で分かったこと

### 1) Locker なしの移動は失敗する

`exit -> thread move -> enter -> compile/run` を Locker なしで行うと、移動先スレッドの最初の実行で失敗しました。  
再現テストでは `Uncaught RangeError: Maximum call stack size exceeded`（`Script::run()` が `None`）を確認。

ポイント:

- 失敗は「既存 `Global<Context>` の reopen 問題」だけではない
- **移動先で fresh `Context` を作っても失敗**する
- つまり根本は context 再利用以前に「スレッド再入場の契約」側

### 2) Locker ありでも最初は SIGSEGV していた

最終的な原因は、Rust 側 `Locker::new` の FFI で  
**`v8::Isolate*` ではなく Rust wrapper (`&Isolate`) のアドレスを渡していたこと**でした。

- 誤: `isolate as *const Isolate as *mut Isolate`
- 正: `isolate.as_real_ptr()`（`*mut RealIsolate`）

この修正後、Locker の same-thread smoke が通り、cross-thread migration も通るようになりました。

## 実装した API / 変更点

### 公開 API

- `Locker`（`src/locker.rs`）
  - `v8::Locker` の Rust ラッパー
- `SendableOwnedIsolate`（`src/sendable.rs`）
  - `unsafe` で `OwnedIsolate` を `Send` として包む
- `SendableGlobal<T>`（`src/sendable.rs`）
  - `unsafe` で `Global<T>` を `Send` として包む
- `Isolate::new_sendable(params)`（`src/isolate.rs`）
  - 送信用ラッパーを返すコンストラクタ

### ネイティブ shim / ビルド

- `src/locker.cc`
  - `v8__Locker__CONSTRUCT` / `v8__Locker__DESTRUCT` を提供
- `src/v8_headers/v8-locker.h`, `src/v8_headers/v8config.h`
  - V8 `14.9.207.2` 対応ヘッダを利用
- `build.rs`
  - `cc::Build` で `src/locker.cc` をコンパイル
- `Cargo.toml`
  - `build-dependencies` に `cc = "1"` を追加

### テスト

`src/lib.rs` に以下を追加:

- `isolate_can_call_js_with_locker_same_thread`
- `isolate_can_migrate_threads_with_fresh_contexts_and_locker`
- `isolate_can_migrate_threads_and_call_js_with_persistent_context_and_locker`
- Locker なし失敗再現テスト（`#[ignore]` で記録）

また、テスト全体の初期化競合を避けるため `array_buffer` 側を `crate::initialize_v8()` に統一。

## 使い方（最小パターン）

```rust
let mut isolate = v8::Isolate::new_sendable(Default::default()).into_inner();

// thread A
{
  let _locker = v8::Locker::new(&isolate);
  unsafe { isolate.enter() };
  // JS 実行
  unsafe { isolate.exit() }; // handoff 前に exit
}
let isolate = unsafe { v8::SendableOwnedIsolate::new(isolate) };

// thread B
let handle = std::thread::spawn(move || {
  let mut isolate = isolate.into_inner();
  let _locker = v8::Locker::new(&isolate);
  unsafe { isolate.enter() };
  // JS 実行
  // 次スレッドへ渡すなら exit して sendable で包み直す
  unsafe { isolate.exit() };
  unsafe { v8::SendableOwnedIsolate::new(isolate) }
});
```

## 重要な運用ルール

- `SendableOwnedIsolate::new()` で包む前に `exit` しておく
- 再入場するスレッドでは `Locker` を取り、`enter` してから V8 API を使う
- 最終的に dispose するスレッドでは `OwnedIsolate` の既存 drop 契約を守る
  - （handoff の中間点では `exit`、最終破棄側では entered 状態で drop される流れ）

## 移植チェックリスト（元プロジェクト向け）

1. `Locker` FFI が **必ず `*mut RealIsolate`** を受け取っていることを確認
2. `locker.cc` のビルド設定（`cc` + include path）を追加
3. `SendableOwnedIsolate` / `SendableGlobal<T>` を追加（または同等の unsafe wrapper を実装）
4. handoff 点の `exit` / 再入場点の `Locker + enter` を全経路で確認
5. 以下 3 種類のテストを通す
   - same-thread + Locker
   - fresh context での cross-thread
   - persistent `Global<Context>` での cross-thread

## 検証結果（このフォーク）

- `cargo test --lib`  
  - **16 passed, 0 failed, 2 ignored**
  - ignored は「Locker なし失敗モードの記録用」

