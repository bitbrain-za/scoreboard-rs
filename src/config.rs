use log::debug;

#[derive(Debug)]
pub enum RunMode {
    Update(WriteConfig),
    Read(ReadConfig),
}

impl RunMode {
    pub fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mode = if args.contains(&String::from("-n")) {
            RunMode::Update(WriteConfig::from_args(args)?)
        } else {
            RunMode::Read(ReadConfig::from_args(args)?)
        };
        Ok(mode)
    }
}

#[derive(Debug)]
pub struct ReadConfig {
    pub table: String,
    pub sort: String,
    pub limit: Option<usize>,
}

impl ReadConfig {
    fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut table: Option<String> = None;
        let mut sort = String::from("time_ns");
        let mut limit = None;

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-t" => {
                    table = Some(args.get(i + 1).ok_or("-t needs a parameter")?.to_string());
                }
                "-s" => {
                    sort = args
                        .get(i + 1)
                        .ok_or("-s must provide a string")?
                        .to_string();
                }
                "-n" => {
                    limit = args
                        .get(i + 1)
                        .ok_or("-l must provide a string")?
                        .parse::<usize>()
                        .ok();
                }
                _ => {}
            }
        }

        let config = ReadConfig {
            table: table.ok_or("-t must be provided")?,
            sort,
            limit,
        };

        debug!("Read Config: {:?}", config);
        Ok(config)
    }
}

#[derive(Debug)]
pub struct WriteConfig {
    pub table: String,
    pub name: String,
    pub command: String,
    pub time_ns: u32,
}

impl WriteConfig {
    fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut name: Option<String> = None;
        let mut command: Option<String> = None;
        let mut time_ns: Option<u32> = None;
        let mut table: Option<String> = None;

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-n" => {
                    name = Some(
                        args.get(i + 1)
                            .ok_or("-n must provide a string")?
                            .to_string(),
                    );
                }
                "-c" => {
                    command = Some(
                        args.get(i + 1)
                            .ok_or("-c must provide a string")?
                            .to_string(),
                    );
                }
                "-r" => {
                    time_ns = Some(
                        args.get(i + 1)
                            .ok_or("-r must provide an argument")?
                            .parse::<u32>()
                            .map_err(|e| format!("-r must be a integer: {}", e))?,
                    );
                }
                "-t" => {
                    table = Some(
                        args.get(i + 1)
                            .ok_or("-t must provide a string")?
                            .to_string(),
                    );
                }
                _ => {}
            }
        }

        let config = WriteConfig {
            table: table.ok_or("-t must be provided")?,
            name: name.ok_or("-n must be provided")?,
            command: command.ok_or("-c must be provided")?,
            time_ns: time_ns.ok_or("-r must be provided")?,
        };

        debug!("Write Config: {:?}", config);
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_config() {
        let args = vec![
            String::from("-t"),
            String::from("table"),
            String::from("-s"),
            String::from("sort"),
            String::from("-n"),
            String::from("1"),
        ];
        let config = ReadConfig::from_args(&args).expect("Error parsing args");
        assert_eq!(config.table, "table");
        assert_eq!(config.sort, "sort");
        assert_eq!(config.limit, Some(1));
    }

    #[test]
    fn test_read_config_missing_table() {
        let args = vec![
            String::from("-s"),
            String::from("sort"),
            String::from("-n"),
            String::from("1"),
        ];
        let config = ReadConfig::from_args(&args);
        assert!(config.is_err());
        assert_eq!(
            config.err().unwrap().to_string(),
            "-t must be provided".to_string()
        );
    }

    #[test]
    fn test_write_config() {
        let args = vec![
            String::from("-t"),
            String::from("table"),
            String::from("-n"),
            String::from("name"),
            String::from("-c"),
            String::from("command"),
            String::from("-r"),
            String::from("1"),
        ];
        let config = WriteConfig::from_args(&args).expect("Error parsing args");
        assert_eq!(config.table, "table");
        assert_eq!(config.name, "name");
        assert_eq!(config.command, "command");
        assert_eq!(config.time_ns, 1);
    }

    #[test]
    fn test_write_config_missing_table() {
        let args = vec![
            String::from("-n"),
            String::from("name"),
            String::from("-c"),
            String::from("command"),
            String::from("-r"),
            String::from("1"),
        ];
        let config = WriteConfig::from_args(&args);
        assert!(config.is_err());
        assert_eq!(
            config.err().unwrap().to_string(),
            "-t must be provided".to_string()
        );
    }

    #[test]
    fn test_write_config_missing_name() {
        let args = vec![
            String::from("-t"),
            String::from("table"),
            String::from("-c"),
            String::from("command"),
            String::from("-r"),
            String::from("1"),
        ];
        let config = WriteConfig::from_args(&args);
        assert!(config.is_err());
        assert_eq!(
            config.err().unwrap().to_string(),
            "-n must be provided".to_string()
        );
    }

    #[test]
    fn test_write_config_missing_command() {
        let args = vec![
            String::from("-t"),
            String::from("table"),
            String::from("-n"),
            String::from("name"),
            String::from("-r"),
            String::from("1"),
        ];
        let config = WriteConfig::from_args(&args);
        assert!(config.is_err());
        assert_eq!(
            config.err().unwrap().to_string(),
            "-c must be provided".to_string()
        );
    }
}
