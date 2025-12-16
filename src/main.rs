// シングルスレッドの非同期処理の簡単な例
// ※ tokio クレートを使うバージョンです（ランタイムは 1 スレッド）。

use tokio::time::{sleep, Duration};

// 疑似的な「重い処理」: 少しずつメッセージを出しながら非同期で待つ
async fn task(name: &str) {
	for i in 0..5 {
		println!("{}: ステップ {}", name, i);
		// ここで非同期に待つ（スレッドは増えない）
		sleep(Duration::from_millis(200)).await;
	}
}

// シングルスレッドランタイム（current_thread）で動かす
#[tokio::main(flavor = "current_thread")]
async fn main() {
	println!("開始（シングルスレッド非同期）");

	// 2つの非同期タスクを同時進行させる（でもスレッドは 1 本）
	let t1 = task("タスクA");
	let t2 = task("タスクB");
    let t3 = task("タスクC");

	// どちらも終わるまで待つ
	tokio::join!(t1, t2, t3);
	println!("終了");
}

