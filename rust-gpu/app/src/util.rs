// Returns a bool if the the DEBUG_LAYER env is not 0 / false
pub fn enable_debug_layer() -> bool {
    std::env::var("DEBUG_LAYER").is_ok_and(|e| !(e == "0" || e == "false"))
}
