use eframe::egui;
use std::fs;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 500.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "微分積分方程式 ソルバー",
        options,
        Box::new(|_cc| {
            _cc.egui_ctx.set_visuals(egui::Visuals::dark());

            // --- 💡 日本語フォントの設定を追加 ---
            setup_custom_fonts(&_cc.egui_ctx);

            Box::new(SolverApp::default())
        }),
    )
}

// 日本語フォントをシステムから読み込んでeguiに登録する関数
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Windowsの日本語標準フォントのパス（環境に合わせて変更してください）
    // Macの場合の例: "/System/Library/Fonts/Hinted/HiraginoSans-W3.ttc"
    let font_path = "C:\\Windows\\Fonts\\msgothic.ttc"; 

    if let Ok(font_data) = fs::read(font_path) {
        // フォントデータをeguiに登録
        fonts.font_data.insert(
            "japanese_font".to_owned(),
            egui::FontData::from_owned(font_data),
        );

        // プロポーショナル（標準）とモノスペース（等幅）の両方に日本語フォントを最優先で割り当て
        fonts.families.get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "japanese_font".to_owned());
            
        fonts.families.get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "japanese_font".to_owned());

        // 設定をコンテキストに反映
        ctx.set_fonts(fonts);
    } else {
        println!("警告: 指定されたパスにフォントが見つかりませんでした。デフォルトフォントを使用します。");
    }
}

// --- 以下はこれまでのロジックのまま不変です ---

struct SolverApp {
    target_x: f64,
    coeff_k: f64,
    coeff_lambda: f64,
    res_f: f64,
    res_integral: f64,
    res_derivative: f64,
}

impl Default for SolverApp {
    fn default() -> Self {
        Self {
            target_x: 1.0,
            coeff_k: 1.0,
            coeff_lambda: 1.0,
            res_f: 0.0,
            res_integral: 0.0,
            res_derivative: 1.0,
        }
    }
}

impl SolverApp {
    fn calculate(&mut self) {
        if self.target_x <= 0.0 {
            self.res_f = 0.0;
            self.res_integral = 0.0;
            self.res_derivative = self.coeff_k;
            return;
        }

        let n = 1000;
        let h = self.target_x / (n as f64);
        let mut f_vals = vec![0.0; n + 1];
        f_vals[0] = 0.0;

        for i in 0..n {
            let mut current_integral = 0.0;
            if i > 0 {
                for j in 0..i {
                    current_integral += 0.5 * h * (f_vals[j] + f_vals[j + 1]);
                }
            }
            let derivative_val = self.coeff_k - (self.coeff_lambda * current_integral);
            f_vals[i + 1] = f_vals[i] + h * derivative_val;
        }

        let mut final_integral = 0.0;
        for j in 0..n {
            final_integral += 0.5 * h * (f_vals[j] + f_vals[j + 1]);
        }

        self.res_f = f_vals[n];
        self.res_integral = final_integral;
        self.res_derivative = self.coeff_k - (self.coeff_lambda * final_integral);
    }
}

impl eframe::App for SolverApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("微分積分方程式 数値シミュレータ");
            ui.label("方程式: f'(x) = k - λ ∫₀ˣ f(t) dt  [初期値 f(0)=0]");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("ターゲット x の値:");
                ui.add(egui::DragValue::new(&mut self.target_x).speed(0.05).clamp_range(0.0..=6.28));
            });
            ui.small("※0.0 〜 6.28 (2π) の間で調整してください");

            ui.add(egui::Slider::new(&mut self.coeff_k, 0.1..=5.0).text("係数 k (外部入力)"));
            ui.add(egui::Slider::new(&mut self.coeff_lambda, 0.1..=5.0).text("係数 λ (積分の重み)"));

            ui.add_space(10.0);

            if ui.button("⚡ 方程式を解く").clicked() || ctx.input(|i| i.pointer.any_down()) {
                self.calculate();
            }

            ui.separator();
            ui.heading("📊 計算結果（それぞれの解）");
            ui.add_space(5.0);

            egui::Grid::new("results_grid")
                .num_columns(2)
                .spacing([40.0, 15.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("関数値 f(x) の解:");
                    ui.colored_label(egui::Color32::LIGHT_BLUE, format!("{:.6}", self.res_f));
                    ui.end_row();

                    ui.label("積分の解 ( ∫₀ˣ f(t)dt ):");
                    ui.colored_label(egui::Color32::LIGHT_GREEN, format!("{:.6}", self.res_integral));
                    ui.end_row();

                    ui.label("微分の解 f'(x):");
                    ui.colored_label(egui::Color32::LIGHT_RED, format!("{:.6}", self.res_derivative));
                    ui.end_row();
                });

            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label("💡 ");
                ui.small("使い方: スライダーや数値を動かした後、ボタンを押すか画面をドラッグ・クリックすると即座に解が再計算されます。k=1, λ=1 のとき、f(x) は sin(x) に一致します。");
            });
        });
    }
}