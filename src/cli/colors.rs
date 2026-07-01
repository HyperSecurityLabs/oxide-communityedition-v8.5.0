// ----------------------------------------------------------------------------
//  colors.rs — color constants and helpers
// ----------------------------------------------------------------------------
//  Defines the Colors struct with ANSI truecolor helper methods for output
//  formatting (warning, ok, brand). Also provides the print_status function
//  for standardized status messages with severity-based coloring.
//
//  --- Developers ---------------------------------------------------------------
//  khaninkali             — разработчик / core engineer (Rust backend, logic)
//  Lyara Koroleva         — дизайнер / blazing fast CLI & interface design
//  HsecDevelopers         — 测试 / テスト / testing & QA (integration, validation)
//  projectk 2091         — HyperSecurityOffensiveLabs lineage
// ----------------------------------------------------------------------------
//
//
// ---------------------------------------------------------------------------
//   WARNING / 警告 / 警告
// ---------------------------------------------------------------------------
//  This source code is the exclusive property of HyperSecurityOffensiveLabs.
//  You are permitted to VIEW this code for educational and reference
//  purposes only. You may NOT modify, distribute, sublicense, or create
//  derivative works without explicit written permission from khaninkali
//  and the HyperSecurityOffensiveLabs development team.
//
//  このソースコードはHyperSecurityOffensiveLabsの独占的知的財産です
//  教育目的および参照目的での閲覧のみ許可されています
//  khaninkaliおよびHyperSecurityOffensiveLabs開発チームの
//  書面による明示的な許可なく修正配布サブライセンス
//  または二次的著作物の作成を禁止します
//
//  本源代码是HyperSecurityOffensiveLabs的独家财产
//  仅允许出于教育和参考目的查看未经khaninkali和
//  HyperSecurityOffensiveLabs开发团队的书面明确许可，
//  禁止修改分发再许可或创建衍生作品
// ---------------------------------------------------------------------------
//
//
//  カラーパレット定義 / colour palette constants
//  Colors構造体 — ANSI TrueColorヘルパーメソッド / TrueColor helper methods
//    warning() — ラベンダー色の警告テキスト / lavender warning text
//    ok() — ジェード色の成功テキスト / jade success text
//    brand() — 太字ジェードのブランドテキスト / bold jade brand text
//  ステータス表示関数 / status display function
//    print_status() — 重大度に応じた色分けでステータスを出力 / outputs colour-coded status messages
//    "OK"  bright green / 明るい緑
//    "ERROR"  bright red bold / 明るい赤太字
//    "WARN"  bright yellow / 明るい黄
//    "INFO"  bright cyan / 明るいシアン
//    "VULN"  truecolor red bold / 赤太字
//    "CRITICAL"  truecolor red bold / 赤太字
//    _ (default)  汎用フォーマット / generic format
//
use colored::Colorize;

const KIKYO: (u8,u8,u8) = (91,106,191);   // 桔梗
const HISUI_B: (u8,u8,u8) = (80,215,170); // 翡翠明
const SHU: (u8,u8,u8) = (232,57,41);       // 朱
const KURENAI: (u8,u8,u8) = (215,0,58);   // 紅
const YAMABUKI: (u8,u8,u8) = (255,177,27); // 山吹
const TSUYUKUSA: (u8,u8,u8) = (46,169,223); // 露草

pub struct Colors;

impl Colors {
    pub fn warning(text: &str) -> String {
        text.truecolor(KIKYO.0, KIKYO.1, KIKYO.2).to_string()
    }

    pub fn ok(text: &str) -> String {
        text.truecolor(HISUI_B.0, HISUI_B.1, HISUI_B.2).to_string()
    }

    pub fn brand(text: &str) -> String {
        text.truecolor(HISUI_B.0, HISUI_B.1, HISUI_B.2).bold().to_string()
    }
}

pub fn print_status(status: &str, message: &str) {
    match status {
        "OK"       => println!("{} {}", "翡翠".truecolor(56,180,139), message),
        "ERROR"    => println!("{} {}", "朱".truecolor(232,57,41).bold(), message),
        "WARN"     => println!("{} {}", "山吹".truecolor(255,177,27), message),
        "INFO"     => println!("{} {}", "露草".truecolor(46,169,223), message),
        "VULN"     => println!("{} {}", "紅".truecolor(215,0,58).bold(), message),
        "CRITICAL" => println!("{} {}", "朱".truecolor(232,57,41).bold(), message),
        _          => println!("[{:^6}] {}", status, message),
    }
}
