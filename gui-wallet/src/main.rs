use eframe::{egui, NativeOptions};

fn main() -> eframe::Result<()> {
    let app = WalletApp::default();
    eframe::run_native("Obscura Wallet", NativeOptions::default(), Box::new(|_cc| Box::new(app)))
}

#[derive(Default)]
struct WalletApp {
    address: String,
    balance: u64,
    recipient: String,
    amount: String,
}

impl eframe::App for WalletApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Obscura GUI Wallet (placeholder)");
            ui.label(format!("Address: {}", self.address));
            ui.label(format!("Balance: {}", self.balance));
            ui.separator();
            ui.heading("Send Transaction");
            ui.label("Recipient");
            ui.text_edit_singleline(&mut self.recipient);
            ui.label("Amount");
            ui.text_edit_singleline(&mut self.amount);
            if ui.button("Send").clicked() {
                // TODO: call RPC to send
                println!("Sending {} to {}", self.amount, self.recipient);
            }
        });
    }
}
