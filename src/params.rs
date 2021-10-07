use super::*;
use sdb_dyn_proto_rdr::reader::account_with_cfs::Cashflow;

pub struct StaticParams<'a> {
    pub config_params: &'a ConfigurationParameters,
    pub log: &'a Logger,
    pub diag_log: &'a Logger,
    pub old_acc_map: OldAccountMap,
    pub adj_rates: Adjustments,
    pub bal_slab: BalanceSlabs,
    pub spread_writer: BufWriter<File>,
    pub method_master: MethodMap,
    pub saved_bm_rates: IntermediateBMPointsMap,
    pub out_writer: BufWriter<File>,
    pub cf_det_writer: BufWriter<File>,
}

impl<'a> StaticParams<'a> {
    pub fn new(
        log: &'a Logger,
        diag_log: &'a Logger,
        config_params: &'a ConfigurationParameters,
    ) -> Self {
        // Deriving spread file output path
        let spread_path = config_params
            .output_file_path()
            .to_string()
            .replace(".txt", "_spread.txt");

        // Deriving cf level detailed file output path
        let cf_dt_path = config_params
            .output_file_path()
            .to_string()
            .replace(".txt", "_cf_det.txt");
        let mut cf_det_writer = get_writer(&cf_dt_path);

        // Writing header in cf detailed file
        write!(
            cf_det_writer,
            "AccountId|CashflowDate|ResidualDays|PrincipalAmount|YieldRate|FixedAdjustment|OriginalTenorBalance|BaseRateProduct|EndRateProduct\n",
        )
        .expect("Error while writing headers in cf_level detail file.");

        Self {
            config_params,
            log,
            diag_log,
            old_acc_map: SpreadReader::new(config_params.spread_file_path()),
            adj_rates: Adjustments::new(config_params.adj_rate_file_path()),
            bal_slab: BalanceSlabs::new(config_params.bal_slab_file()),
            spread_writer: get_writer(&spread_path),
            method_master: get_method_config(&MethodField::new_from_path(
                &config_params.method_req_fields_file_path(),
            )),
            saved_bm_rates: HashMap::new(),
            out_writer: get_writer(config_params.output_file_path()),
            cf_det_writer,
        }
    }
}

pub struct DynamicParams {
    pub is_consolidated: bool,
    pub m_rules: AggRules,
    pub bc_rules: AggRules,
    pub fix_adj_rules: AggRulesAdj,
    pub var_adj_rules: AggRulesAdj,
    pub input_field_names: AccFieldNames,
    pub avg_bal: AverageBalance,
    pub exrt_map: ExchangeRates,
    pub out_writer: BufWriter<File>,
    pub is_cf_req: bool,
}

impl<'a> DynamicParams {
    pub fn new(config_params: &'a ConfigurationParameters, file: &'a ConfigFile) -> Self {
        let input_data = Reader::new_at_path(
            &file.metadata_file_path,
            &get_file_path(file.input_file_path.to_string(), *config_params.to_date()),
        );

        let amb_file_path = get_file_path(file.amb_file_path.to_string(), *config_params.to_date());

        Self {
            is_consolidated: file.is_consolidated,
            m_rules: AggRules::new_from_path(config_params.method_rules_file_path(), &input_data),
            bc_rules: AggRules::new_from_path(config_params.bc_rule_file_path(), &input_data),
            fix_adj_rules: AggRulesAdj::new_from_path(
                config_params.fix_adj_rule_file_path(),
                &input_data,
            ),
            var_adj_rules: AggRulesAdj::new_from_path(
                config_params.var_adj_rule_file_path(),
                &input_data,
            ),
            input_field_names: AccFieldNames::new_from_path(&file.req_fields_file_path),
            avg_bal: AverageBalance::new(&amb_file_path),
            exrt_map: ExchangeRates::new(&get_file_path(
                file.exrt_file_path.to_string(),
                *config_params.to_date(),
            )),
            out_writer: get_writer(config_params.output_file_path()),
            is_cf_req: file.is_cf_req,
        }
    }
}

pub type Cashflows = Vec<Cashflow>;
pub struct DerivedFields {
    pub method_id: i32,
    pub parsed_method: ParsedMethod,
    pub basecurve: i32,
    pub fix_adjs: Vec<i32>,
    pub var_adjs: Vec<i32>,
    pub cashflows: Cashflows,
}

impl DerivedFields {
    pub fn new(
        acc_id: &str,
        acc_data: &mut AccountWithCFs,
        static_params: &StaticParams,
        dyn_params: &DynamicParams,
    ) -> Self {
        let method_id = get_method(
            acc_id,
            &acc_data,
            &dyn_params.m_rules,
            static_params.config_params.default_method(),
            &static_params.log,
        );

        Self {
            method_id,
            parsed_method: ParsedMethod::new(
                method_id,
                &static_params.method_master,
                acc_data,
                timestamp(*static_params.config_params.to_date()),
            ),
            basecurve: get_bc(
                acc_id,
                &acc_data,
                &dyn_params.bc_rules,
                static_params.config_params.default_basecurve(),
                &static_params.log,
            ),
            fix_adjs: get_adj(
                acc_id,
                &acc_data,
                &dyn_params.fix_adj_rules,
                static_params.config_params.fixed_adj_count(),
                &static_params.log,
            ),
            var_adjs: get_adj(
                acc_id,
                &acc_data,
                &dyn_params.var_adj_rules,
                static_params.config_params.var_adj_count(),
                &static_params.log,
            ),
            cashflows: acc_data
                .remove_cfs_for_key(&dyn_params.input_field_names.cashflows)
                .unwrap_or_default(),
        }
    }
}
