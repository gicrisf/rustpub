use std::process::Command;
use crate::parse::ParserKind;

pub struct ReadabiliPyCmd {}

impl ReadabiliPyCmd {
    pub fn json_from_file(parser: ParserKind, html_fpath: String, json_fpath: String) -> String {

        let parser_arg = match parser {
            ParserKind::ReadabiliPy => { "-p" },
            ParserKind::ReadabilityJs => { "" },
            _ => "",
        };

        let arg = format!(
            r#"readabilipy {parser} -i {in} -o {out}"#,
            parser = parser_arg,
            in = html_fpath,
            out = json_fpath,
        );

        // Launch command. TODO: Add to trait for all commands!
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").arg("/C").arg(&arg).output()
            .expect("Windows failed to execute send cmd")
        } else {
            Command::new("sh").arg("-c").arg(&arg).output()
            .expect("Linux failed to execute send cmd")
        };

        // Shell output
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
