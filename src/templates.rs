const HOOK_TEMPLATE: &str = include_str!("./templates/hook.sh");
const CONFIG_TEMPLATE: &str = include_str!("./templates/config.yml");

pub fn render_hook(hook_name: &str) -> String {
    HOOK_TEMPLATE.replace("{{hook_name}}", hook_name)
}

pub fn render_config() -> String {
    CONFIG_TEMPLATE.to_string()
}
