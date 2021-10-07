mod ttl_bals;
pub use self::ttl_bals::*;
use super::*;

fn get_base_curve_file_path(
    mut one_acc_out: &mut OneAccountView,
    static_params: &StaticParams,
    derived_fields: &DerivedFields,
) -> (BMKey, String) {
    let bm_key = BMKey {
        date: derived_fields
            .parsed_method
            .curve_pick_date
            .format("%d-%m-%Y")
            .to_string(),
        base_curve_id: derived_fields.basecurve,
    };
    one_acc_out.base_rate_curve_id = bm_key.base_curve_id;

    // Deriving basecurve file path
    let full_file_path = format!(
        "{}{}_{}.txt",
        static_params.config_params.bc_file_path(),
        bm_key.date,
        bm_key.base_curve_id
    );
    (bm_key, full_file_path)
}

pub fn calc_bm_rates_acc_level(
    mut one_acc_out: &mut OneAccountView,
    static_params: &mut StaticParams,
    derived_fields: &DerivedFields,
    ttl_bals: &mut TotalBalances,
) {
    let mut lst_bm: IntermediateBMPoints = Vec::new();
    let (bm_key, full_file_path) =
        get_base_curve_file_path(one_acc_out, static_params, derived_fields);

    if Path::new(&full_file_path).exists() {
        if let Some(prev_bm_rate) = static_params.saved_bm_rates.get(&bm_key) {
            lst_bm = prev_bm_rate.to_vec();
        } else {
            bm_reader::get_bm_points(
                &full_file_path,
                derived_fields.parsed_method.curve_pick_date,
                &mut lst_bm,
            );
            static_params.saved_bm_rates.insert(bm_key, lst_bm.clone());
        }

        if derived_fields.parsed_method.tenor_end_date
            > derived_fields.parsed_method.curve_pick_date
        {
            ttl_bals.residual_days = num_days_start_to_end(
                derived_fields.parsed_method.curve_pick_date,
                derived_fields.parsed_method.tenor_end_date,
            );
        }
        ig_neg_val_i64(&mut ttl_bals.residual_days);
        let mut yield_rate: f64 =
            calc_yield_rate(&mut lst_bm, ttl_bals.residual_days).unwrap_or(DEFAULT_FLOAT);
        ig_neg_val_f64(&mut yield_rate);

        one_acc_out.base_rate = yield_rate as f32;
    } else {
        log_debug!(
            static_params.log,
            "Benchmark file not found at path: {} for account: {}.",
            full_file_path,
            one_acc_out.account_id
        );
    }
}

pub fn calc_bm_rates_cf_level(
    one_acc_out: &mut OneAccountView,
    mut static_params: &mut StaticParams,
    mut dyn_params: &mut DynamicParams,
    mut derived_fields: &mut DerivedFields,
    mut ttl_bals: &mut TotalBalances,
) {
    let mut lst_bm: IntermediateBMPoints = Vec::new();

    let (bm_key, full_file_path) =
        get_base_curve_file_path(one_acc_out, static_params, derived_fields);
    if Path::new(&full_file_path).exists() {
        if let Some(prev_bm_rate) = static_params.saved_bm_rates.get(&bm_key) {
            lst_bm = prev_bm_rate.to_vec();
        } else {
            bm_reader::get_bm_points(
                &full_file_path,
                derived_fields.parsed_method.curve_pick_date,
                &mut lst_bm,
            );
            static_params.saved_bm_rates.insert(bm_key, lst_bm.clone());
        }
        get_data_from_cfs(
            &mut derived_fields,
            &mut ttl_bals,
            &mut lst_bm,
            &mut static_params,
            &mut dyn_params,
            &one_acc_out.account_id,
        );
    } else {
        log_debug!(
            static_params.log,
            "Benchmark file not found at path: {} for account: {}.",
            full_file_path,
            one_acc_out.account_id
        );

        get_data_from_cfs(
            &mut derived_fields,
            &mut ttl_bals,
            &mut lst_bm,
            &mut static_params,
            &mut dyn_params,
            &one_acc_out.account_id,
        );
    }
}

pub fn get_data_from_cfs(
    derived_fields: &mut DerivedFields,
    ttl_bals: &mut TotalBalances,
    mut lst_bm: &mut IntermediateBMPoints,
    static_params: &mut StaticParams,
    dyn_params: &mut DynamicParams,
    acc_id: &str,
) {
    for cf in derived_fields.cashflows.iter_mut() {
        let cf_date = date_from_timestamp(cf.date);
        if cf_date > derived_fields.parsed_method.curve_pick_date {
            ttl_bals.residual_days = num_days_start_to_end(
                derived_fields.parsed_method.curve_pick_date,
                derived_fields.parsed_method.tenor_end_date,
            );
        }
        ig_neg_val_i64(&mut ttl_bals.residual_days);
        let mut yield_rate: f64 =
            calc_yield_rate(&mut lst_bm, ttl_bals.residual_days).unwrap_or(DEFAULT_FLOAT);
        ig_neg_val_f64(&mut yield_rate);

        ttl_bals.ttl_prin_amt += cf.principal_amount;
        ttl_bals.ttl_int_amt += cf.interest_amount;
        let org_bal_tenor = ttl_bals.residual_days as f64 * cf.principal_amount;
        let base_rate_prod = yield_rate * org_bal_tenor;
        let end_rate_prod = (yield_rate + ttl_bals.fix_adj) * org_bal_tenor;
        ttl_bals.ttl_base_rate_prod += base_rate_prod;
        ttl_bals.ttl_end_rate_bal += end_rate_prod;
        ttl_bals.ttl_org_tenor_bal += org_bal_tenor;
        ttl_bals.ttl_ftp_amt +=
            ttl_bals.ttl_base_rate_prod / (ttl_bals.max_days_in_year as f64 * 100.0);

        if dyn_params.is_cf_req {
            write!(
                static_params.cf_det_writer,
                "{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                acc_id,
                date_from_timestamp(cf.date).format("%d-%m-%Y"),
                ttl_bals.residual_days,
                cf.principal_amount,
                yield_rate,
                ttl_bals.fix_adj,
                org_bal_tenor,
                base_rate_prod,
                end_rate_prod,
            )
            .expect("Error while writing cf_level details.");
        }
    }
}
