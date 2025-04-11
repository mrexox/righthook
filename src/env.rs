use std::env::var;
use std::sync::LazyLock;

pub static RIGHTHOOK_VERBOSE: LazyLock<bool> = LazyLock::new(|| {
    var_bool("RIGHTHOOK_VERBOSE")
        .or(var_bool("RIGHTHOOK_DEBUG"))
        .unwrap_or(false)
});

fn var_bool(key: &str) -> Option<bool> {
    var(key)
        .ok()
        .map(|value| value.to_lowercase())
        .map(|value| value != "0" && value != "false" && value != "no")
}
