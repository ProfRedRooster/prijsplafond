use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Polygon};
use std::f64::consts::PI;

struct PriceLimitSimulator {
    price_limit: f64,
    supply_shift: f64,
    demand_shift: f64,
}

impl Default for PriceLimitSimulator {
    fn default() -> Self {
        Self {
            price_limit: 10.0,
            supply_shift: 0.0,
            demand_shift: 0.0,
        }
    }
}

impl PriceLimitSimulator {
    fn supply_formula(&self, quantity: f64) -> f64 {
        (0.5 * quantity + 5.0 + self.supply_shift).max(0.0)
    }

    fn demand_formula(&self, quantity: f64) -> f64 {
        (-0.5 * quantity + 15.0 + self.demand_shift).max(0.0)
    }

    fn calculate_surplus(&self) -> (f64, f64, f64, f64) {
        let equilibrium_quantity = (15.0 + self.demand_shift - (5.0 + self.supply_shift)).max(0.0);
        let equilibrium_price = self.supply_formula(equilibrium_quantity);
        let mut actual_price = equilibrium_price;
        let mut actual_quantity = equilibrium_quantity;

        if self.price_limit < equilibrium_price {
            actual_price = self.price_limit;
            actual_quantity = (self.price_limit - (5.0 + self.supply_shift)).max(0.0) / 0.5;
        }

        let consumer_surplus = 0.5 * actual_quantity * (15.0 + self.demand_shift - actual_price);
        let producer_surplus = 0.5 * actual_quantity * (actual_price - (5.0 + self.supply_shift));

        (consumer_surplus, producer_surplus, actual_quantity, actual_price)
    }
}

impl eframe::App for PriceLimitSimulator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Price Limit Simulator");
            ui.add(egui::Slider::new(&mut self.price_limit, 0.0..=20.0).text("Price Limit"));
            ui.add(egui::Slider::new(&mut self.supply_shift, -10.0..=10.0).text("Supply Shift"));
            ui.add(egui::Slider::new(&mut self.demand_shift, -10.0..=10.0).text("Demand Shift"));

            let (consumer_surplus, producer_surplus, eq_qty, eq_price) = self.calculate_surplus();
            ui.label(format!("Consumer Surplus: {:.2}", consumer_surplus));
            ui.label(format!("Producer Surplus: {:.2}", producer_surplus));

            Plot::new("price_quantity_graph").show(ui, |plot_ui| {
                let supply: PlotPoints = (0..100)
                    .map(|x| {
                        let quantity = x as f64 / 5.0;
                        [quantity, self.supply_formula(quantity)]
                    })
                    .collect();

                let demand: PlotPoints = (0..100)
                    .map(|x| {
                        let quantity = x as f64 / 5.0;
                        [quantity, self.demand_formula(quantity)]
                    })
                    .collect();

                let price_limit_line: PlotPoints = vec![
                    [0.0, self.price_limit],
                    [20.0, self.price_limit],
                ]
                    .into();

                let consumer_surplus_area = Polygon::new(vec![
                    [0.0, self.demand_formula(0.0)],
                    [eq_qty, self.demand_formula(eq_qty)],
                    [eq_qty, eq_price],
                    [0.0, eq_price],
                ])
                    .fill_color(egui::Color32::LIGHT_BLUE);

                let producer_surplus_area = Polygon::new(vec![
                    [0.0, eq_price],
                    [eq_qty, eq_price],
                    [eq_qty, self.supply_formula(eq_qty)],
                    [0.0, self.supply_formula(0.0)],
                ])
                    .fill_color(egui::Color32::LIGHT_RED);

                plot_ui.polygon(consumer_surplus_area);
                plot_ui.polygon(producer_surplus_area);
                plot_ui.line(Line::new(supply).name("Supply Curve"));
                plot_ui.line(Line::new(demand).name("Demand Curve"));
                plot_ui.line(Line::new(price_limit_line).name("Price Limit"));
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Price Limit Simulator",
        options,
        Box::new(|_cc| Ok(Box::new(PriceLimitSimulator::default()))),
    )
}
