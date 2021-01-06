use std::process::Command;

pub enum ReadabiliPyParser {
    Python,
    Mozilla,
}

pub struct ReadabiliPyCmd {
    parser: ReadabiliPyParser,
}

impl ReadabiliPyCmd {
    pub fn new(parser: ReadabiliPyParser) -> Self {
        Self {
            parser,
        }
    }

    pub fn json_from_file(&self, html_fpath: String, json_fpath: String) -> String {

        let parser_arg = match self.parser {
            ReadabiliPyParser::Python => { "-p" },
            ReadabiliPyParser::Mozilla => { "" },
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
