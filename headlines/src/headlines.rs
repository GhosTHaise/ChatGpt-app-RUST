use confy;
use eframe::egui::{Context, TopBottomPanel,TextEdit};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct HeadlinesConfig{
    dark_mode : bool,
    api_key : String
}


impl Default for HeadlinesConfig{
    fn default() -> Self {
        Self { dark_mode: Default::default(), api_key: Default::default() }
    }
}

pub struct Headlines{
    pub config : HeadlinesConfig,
    pub api_key_initialized : bool
}

impl Headlines {
    pub fn new() -> Headlines {
        let config : HeadlinesConfig = confy::load("headlines").unwrap_or_default();
        Headlines { 
            api_key_initialized: !config.api_key.is_empty(),
            config
        }
    }

}
pub fn render_message_bottom(ctx : &Context)-> () {
    TopBottomPanel::bottom("message").show(ctx , |ui|{
        ui.horizontal(|ui|{
            let mess = ui.add(TextEdit::singleline(&mut "Ask GhosT ... "));
        });
    });
}   
pub struct NewBotResponse{
    bot : String
}