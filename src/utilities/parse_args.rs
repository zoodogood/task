use std::env::Args;
pub struct CliParamsParsed {
    pub command: Option<String>,
    pub params: Vec<String>,
}

pub fn parse(args: &mut Args) -> CliParamsParsed {
    let command: Option<String> = args.nth(0);
    let params: Vec<String> = args.skip(1).collect();
	 CliParamsParsed {command, params}
}
