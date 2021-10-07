use chrono::Datelike;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_args_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    from_date: NaiveDate,
    to_date: NaiveDate,
    config_file_path: String,
    output_file_path: String,
    method_rules_file_path: String,
    bc_rule_file_path: String,
    fix_adj_rule_file_path: String,
    var_adj_rule_file_path: String,
    bc_file_path: String,
    spread_file_path: String,
    adj_rate_file_path: String,
    bal_slab_file: String,
    default_method: i32,
    default_basecurve: i32,
    fixed_adj_count: i32,
    var_adj_count: i32,
    log_file_path: String,
    diagnostics_file_path: String,
    method_req_fields_file_path: String,
    log_level: String,
    ccy: String,
    rate_prec: i8,
    bal_prec: i8,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "from_date:{}", self.from_date());
        info!(logger, "to_date:{}", self.to_date());
        info!(logger, "config_file: {}", self.config_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(
            logger,
            "method_req_fields_file_path: {}",
            self.method_req_fields_file_path()
        );
        info!(logger, "ccy: {}", self.ccy());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(
            logger,
            "method_rule_file: {}",
            self.method_rules_file_path()
        );
        info!(logger, "bc_rule_file: {}", self.bc_rule_file_path());
        info!(logger, "adj_rule_file: {}", self.fix_adj_rule_file_path());
        info!(
            logger,
            "var_adj_rule_file: {}",
            self.var_adj_rule_file_path()
        );
        info!(logger, "bc_file: {}", self.bc_file_path());
        info!(logger, "ftp_rates_file: {}", self.spread_file_path());
        info!(logger, "adj_rate_file_path: {}", self.adj_rate_file_path());
        info!(logger, "bal_slab_file: {}", self.bal_slab_file());
        info!(logger, "default_method: {}", self.default_method());
        info!(logger, "default_basecurve: {}", self.default_basecurve());
        info!(logger, "fixed_adj_count: {}", self.fixed_adj_count());
        info!(logger, "var_adj_count: {}", self.var_adj_count());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "rate_prec: {}", self.rate_prec());
        info!(logger, "bal_prec: {}", self.bal_prec());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let is_def_from_date = matches
            .value_of("is_def_from_date")
            .expect("Error getting `is_def_from_date` value.")
            .parse::<bool>()
            .expect("Cannot parse `is_def_from_date` value as bool.");

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let to_date = date_parser.parse(
            matches
                .value_of("to_date")
                .expect("Error getting `to_date` value."),
        );

        let from_date = if is_def_from_date {
            NaiveDate::from_ymd(to_date.year(), to_date.month(), 01)
        } else {
            date_parser.parse(
                matches
                    .value_of("from_date")
                    .expect("Error getting `from_date` value."),
            )
        };

        let config_file_path = matches
            .value_of("config_file")
            .expect("Error getting `config_file_path` value.")
            .to_string();
        let method_rules_file_path = matches
            .value_of("method_rule_file")
            .expect("Error getting `method_rules_file_path` value.")
            .to_string();
        let bc_rule_file_path = matches
            .value_of("bc_rule_file")
            .expect("Error getting `bc_rule_file_path` value.")
            .to_string();
        let fix_adj_rule_file_path = matches
            .value_of("fix_adj_rule_file")
            .expect("Error getting `fix_adj_rule_file_path` value.")
            .to_string();
        let var_adj_rule_file_path = matches
            .value_of("var_adj_rule_file")
            .expect("Error getting `var_adj_rule_file_path` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_file_path` value.")
            .to_string();
        let method_req_fields_file_path = matches
            .value_of("method_req_fields_file_path")
            .expect("Error getting `method_req_fields_file_path` value.")
            .to_string();
        let bc_file_path = matches
            .value_of("bc_file")
            .expect("Error getting `bc_file_path` value.")
            .to_string();
        let spread_file_path = matches
            .value_of("spread_file_path")
            .expect("Error getting `spread_file_path` value.")
            .to_string();
        let adj_rate_file_path = matches
            .value_of("adj_rate_file_path")
            .expect("Error getting `adj_rate_file_path` value.")
            .to_string();
        let bal_slab_file = matches
            .value_of("bal_slab_file")
            .expect("Error getting `bal_slab_file` value.")
            .to_string();
        let ccy = matches
            .value_of("ccy")
            .expect("Error getting `ccy` value.")
            .to_string();
        let default_method = matches
            .value_of("default_method")
            .expect("Error getting `default_method` value.")
            .to_string()
            .parse::<i32>()
            .expect("Cannot parse `default_method` value as bool.");
        let default_basecurve = matches
            .value_of("default_basecurve")
            .expect("Error getting `default_basecurve` value.")
            .to_string()
            .parse::<i32>()
            .expect("Cannot parse `default_basecurve` value as bool.");
        let fixed_adj_count = matches
            .value_of("fixed_adj_count")
            .expect("Error getting `fixed_adj_count` value.")
            .to_string()
            .parse::<i32>()
            .expect("Cannot parse `fixed_adj_count` value as bool.");
        let var_adj_count = matches
            .value_of("var_adj_count")
            .expect("Error getting `var_adj_count` value.")
            .to_string()
            .parse::<i32>()
            .expect("Cannot parse `var_adj_count` value as bool.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let rate_prec = matches
            .value_of("rate_prec")
            .expect("Error getting `rate_prec` value.")
            .parse::<i8>()
            .expect("Cannot parse `rate_prec` value as bool.");
        let bal_prec = matches
            .value_of("bal_prec")
            .expect("Error getting `bal_prec` value.")
            .parse::<i8>()
            .expect("Cannot parse `bal_prec` value as bool.");
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");

        ConfigurationParameters {
            from_date,
            to_date,
            config_file_path,
            output_file_path,
            method_rules_file_path,
            bc_rule_file_path,
            fix_adj_rule_file_path,
            var_adj_rule_file_path,
            bc_file_path,
            spread_file_path,
            adj_rate_file_path,
            bal_slab_file,
            default_method,
            default_basecurve,
            fixed_adj_count,
            var_adj_count,
            log_file_path,
            diagnostics_file_path,
            method_req_fields_file_path,
            log_level,
            ccy,
            rate_prec,
            bal_prec,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn from_date(&self) -> &NaiveDate {
        &self.from_date
    }
    pub fn to_date(&self) -> &NaiveDate {
        &self.to_date
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn method_rules_file_path(&self) -> &str {
        &self.method_rules_file_path
    }
    pub fn bc_rule_file_path(&self) -> &str {
        &self.bc_rule_file_path
    }
    pub fn fix_adj_rule_file_path(&self) -> &str {
        &self.fix_adj_rule_file_path
    }
    pub fn var_adj_rule_file_path(&self) -> &str {
        &self.var_adj_rule_file_path
    }
    pub fn bc_file_path(&self) -> &str {
        &self.bc_file_path
    }
    pub fn spread_file_path(&self) -> &str {
        &self.spread_file_path
    }
    pub fn adj_rate_file_path(&self) -> &str {
        &self.adj_rate_file_path
    }
    pub fn bal_slab_file(&self) -> &str {
        &self.bal_slab_file
    }
    pub fn default_method(&self) -> i32 {
        self.default_method
    }
    pub fn default_basecurve(&self) -> i32 {
        self.default_basecurve
    }
    pub fn fixed_adj_count(&self) -> i32 {
        self.fixed_adj_count
    }
    pub fn var_adj_count(&self) -> i32 {
        self.var_adj_count
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn method_req_fields_file_path(&self) -> &str {
        &self.method_req_fields_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn ccy(&self) -> &str {
        &self.ccy
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn rate_prec(&self) -> i8 {
        self.rate_prec
    }
    pub fn bal_prec(&self) -> i8 {
        self.bal_prec
    }
}

fn get_args_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("2.0.0")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
        .about("This app helps convert inputs to outputs at lightning speed!")
        .arg(
            Arg::with_name("from_date")
                .long("from-date")
                .value_name("DATE")
                .help("Start date of the FTP process date range")
                .required(false)
        )
        .arg(
            Arg::with_name("to_date")
                .long("to-date")
                .value_name("DATE")
                .help("End date of the FTP process date range")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file")
                .long("config-file")
                .value_name("FILE")
                .help("Path to config file that needs to be processed")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("method_rule_file")
                .long("method-rules-file")
                .value_name("FILE")
                .help("Path to the method rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("bc_rule_file")
                .long("bc-rule-file")
                .value_name("FILE")
                .help("Path to the basecurve rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("fix_adj_rule_file")
                .long("fix-adj-rule-file")
                .value_name("FILE")
                .help("Path to the fixed adjustment rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("var_adj_rule_file")
                .long("var-adj-rule-file")
                .value_name("FILE")
                .help("Path to the variable adjustment rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("bc_file")
                .long("bc-file")
                .value_name("FILE")
                .help("Path to the basecurve file")
                .required(true)
        )
        .arg(
            Arg::with_name("spread_file_path")
                .long("spread-file")
                .value_name("FILE")
                .help("Path to the FTP Rates file")
                .required(true)
        )
         .arg(
            Arg::with_name("adj_rate_file_path")
                .long("adj-rates-file")
                .value_name("FILE")
                .help("Path to the adjustment Rates file")
                .required(true)
        )
        .arg(
            Arg::with_name("bal_slab_file")
                .long("bal-slab")
                .value_name("FILE")
                .help("Path to the balance slab Rates file")
                .required(true)
        )
        .arg(
            Arg::with_name("default_method")
                .long("default-method")
                .value_name("Default Method")
                .help("Default method for Finnone Loans")
                .required(true)
        )
        .arg(
            Arg::with_name("default_basecurve")
                .long("default-basecurve")
                .value_name("Default Basecurve")
                .help("Default basecurve for Finnone Loans")
                .required(true)
        )
        .arg(
            Arg::with_name("fixed_adj_count")
                .long("fixed-adjustments-count")
                .value_name("fixed adjustments count")
                .help("count of fixed adjustments")
                .required(true)
        )
        .arg(
            Arg::with_name("var_adj_count")
                .long("var-adjustments-count")
                .value_name("Variable adjustments count")
                .help("Count of variable adjustments")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("method_req_fields_file_path")
                .long("method-req-fields-file-path")
                .value_name("File")
                .help("Method Required Fields File Path")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("ccy")
                .long("ccy")
                .value_name("Currency")
                .help("Currency")
                .required(true)
        )
        .arg(
            Arg::with_name("is_def_from_date")
                .long("is-def-from-date")
                .value_name("Default To Date")
                .help("The flag that decides whether to_date to be considered first day of the month or not.")
                .possible_values(&["true", "false"])
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("rate_prec")
                .long("rate-prec")
                .value_name("Rate Precision")
                .help("The flag that decides the round off factor for rate fields.")
                .default_value("4")
                .required(false)
        )
        .arg(
            Arg::with_name("bal_prec")
                .long("bal-prec")
                .value_name("Balance Precision")
                .help("The flag that decides the round off factor for balance fields.")
                .default_value("4")
                .required(false)
        )
        .get_matches()
}
