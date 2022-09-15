pub fn get_opcode(module: String, name: String) -> Option<&'static str> {
    Some(match (module.as_str(), name.as_str()) {
        ("looks", "say") => "looks_say",
        ("looks", "say_for_secs") => "looks_sayforsecs",
        ("looks", "think") => "looks_think",
        ("looks", "think_for_secs") => "looks_thinkforsecs",
        ("looks", "show") => "looks_show",
        ("looks", "hide") => "looks_hide",
        _ => return None,
    })
}
