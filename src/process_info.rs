const EDITOR: &str = "nvim";

pub fn is_nvim_foreground(process_info_json: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(process_info_json)
        .ok()
        .and_then(|answer| {
            answer["result"]["process_info"]["foreground_processes"]
                .as_array()
                .map(|processes| {
                    processes
                        .iter()
                        .any(|process| process["name"].as_str() == Some(EDITOR))
                })
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_editor_is_found_beside_whatever_launched_it() {
        assert!(is_nvim_foreground(
            r#"{"result":{"process_info":{"foreground_processes":[{"name":"bash"},{"name":"nvim"}]}}}"#
        ));
    }

    #[test]
    fn a_pane_without_the_editor_is_not_the_editor() {
        assert!(!is_nvim_foreground(
            r#"{"result":{"process_info":{"foreground_processes":[{"name":"bash"}]}}}"#
        ));
    }

    #[test]
    fn every_answer_that_proves_nothing_reads_as_not_the_editor() {
        for answer in [
            "",
            "not json",
            "{}",
            r#"{"result":{}}"#,
            r#"{"result":{"process_info":{"foreground_processes":"nvim"}}}"#,
            r#"{"result":{"process_info":{"foreground_processes":[{"name":42}]}}}"#,
            r#"{"error":{"message":"no such pane"}}"#,
            r#"{"result":{"process_info":{"foreground_processes":[{"name":"/bin/nvim"}]}}}"#,
            r#"{"result":{"process_info":{"foreground_processes":[{"name":"NVIM"}]}}}"#,
        ] {
            assert!(!is_nvim_foreground(answer), "{answer:?} read as the editor");
        }
    }
}
