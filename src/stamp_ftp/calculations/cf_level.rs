use super::*;

pub fn stamp_cf_level(
    mut one_acc_out: &mut OneAccountView,
    static_params: &mut StaticParams,
    dyn_params: &mut DynamicParams,
    derived_fields: &mut DerivedFields,
) {
    let (fix_adjs, var_adjs) = stamp_adjs(&mut one_acc_out, &static_params, &derived_fields);
    let mut ttl_bals = TotalBalances::new(fix_adjs as f64, &static_params);

    calc_bm_rates_cf_level(
        one_acc_out,
        static_params,
        dyn_params,
        derived_fields,
        &mut ttl_bals,
    );
    ttl_bals.derive_rates();

    one_acc_out.base_rate = ttl_bals.base_rate;
    one_acc_out.ftp_rate = ttl_bals.ftp_rate + var_adjs;

    let avg_bal = get_avg_bal(&mut one_acc_out, &static_params, &dyn_params);

    one_acc_out.ftp_amt_ccy = calc_int(
        avg_bal,
        one_acc_out.ftp_rate,
        ttl_bals.run_duration as f64 / ttl_bals.max_days_in_year as f64,
    );
    one_acc_out.lock_spread = one_acc_out.int_rate - one_acc_out.base_rate - fix_adjs;

    // Writing spread data into spread file
    write!(
        static_params.spread_writer,
        "{}",
        one_acc_out.print_spread(derived_fields.parsed_method.id)
    )
    .expect("Error while writing spread file.");
}
