const HOOK_TEMPLATE: &str = include_str!("./templates/hook.txt");

pub fn render_hook(hook_name: &str) -> String {
    HOOK_TEMPLATE.replace("{{hook_name}}", hook_name)
}
