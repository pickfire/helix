use serde::Deserialize;

use crate::keymap::Keymaps;

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Config {
    pub theme: Option<String>,
    #[serde(default)]
    pub lsp: LspConfig,
    #[serde(default)]
    pub keys: Keymaps,
    #[serde(default)]
    pub terminal: TerminalConfig,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LspConfig {
    pub display_messages: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TerminalConfig {
    pub mouse: bool,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self { mouse: true }
    }
}

#[test]
fn parsing_keymaps_config_file() {
    use crate::keymap;
    use crate::keymap::Keymap;
    use helix_core::hashmap;
    use helix_view::document::Mode;

    let sample_keymaps = r#"
            [keys.insert]
            y = "move_line_down"
            S-C-a = "delete_selection"

            [keys.normal]
            A-F12 = "move_next_word_end"
        "#;

    assert_eq!(
        toml::from_str::<Config>(sample_keymaps).unwrap(),
        Config {
            keys: Keymaps(hashmap! {
                Mode::Insert => Keymap::new(keymap!({ "Insert mode"
                    "y" => move_line_down,
                    "S-C-a" => delete_selection,
                })),
                Mode::Normal => Keymap::new(keymap!({ "Normal mode"
                    "A-F12" => move_next_word_end,
                })),
            }),
            ..Default::default()
        }
    );
}
