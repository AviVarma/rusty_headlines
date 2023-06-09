use eframe::{egui::{self, Layout, Separator, TopBottomPanel, Context, Button}, epaint::Color32};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct RustyHeadlines {
    articles: Vec<NewsCardData>,
}

struct NewsCardData {
    title: String,
    description: String,
    url: String,
}

impl RustyHeadlines{
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        // Use the custom font.
        setup_custom_fonts(&cc.egui_ctx);


        // Add Dummy articles to the app
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("Article {}", a),
            description: format!("Description of article {}", a),
            url: format!("https://rusty.rs/{}", a),
        });

        RustyHeadlines {
            articles: Vec::from_iter(iter),
        }
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);

            //render title
            let title = format!("• {}", a.title);
            ui.colored_label(WHITE, title);

            // Render description as a button without it's frame
            ui.add_space(PADDING);
            ui.add(egui::Button::new(&a.description).frame(false));

            // Render the URL as a Hyperlink
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.hyperlink_to("Read more ⤴", &a.url);
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub(crate) fn render_top_panel(&self, cc: &Context){
        TopBottomPanel::top("top panel").show(cc, |ui|{
            ui.add_space(10.);
            egui::menu::bar(ui, |ui |{
                ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui|{
                    use egui::special_emojis::OS_WINDOWS;
                    ui.label(egui::RichText::new(OS_WINDOWS).heading()); // TODO: Add a proper icon!
                });

                // controls
                ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui|{
                    let close_btn = ui.add(Button::new("❌"));
                    let refresh_btn = ui.add(Button::new("🔄"));
                    let theme_btn = ui.add(Button::new("🌙"));
                })
            });
            ui.add_space(10.);
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "Helvetica".to_owned(),egui::FontData::from_static(include_bytes!("../resources/Helvetica.ttf")));

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Helvetica".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("Helvetica".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
