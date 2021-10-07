use super::*;

pub fn stamp_margin(
    mut one_acc_out: &mut OneAccountView,
    static_params: &mut StaticParams,
    dyn_params: &mut DynamicParams,
) {
    let adj_rate = static_params
        .bal_slab
        .get_adj_rate(one_acc_out.cust_agg_bal);
    let ttl_bals = TotalBalances::new(adj_rate as f64, &static_params);
    one_acc_out.ftp_rate = one_acc_out.int_rate + adj_rate;

    let avg_bal = get_avg_bal(&mut one_acc_out, &static_params, &dyn_params);

    one_acc_out.ftp_amt_ccy = calc_int(
        avg_bal,
        one_acc_out.ftp_rate,
        ttl_bals.run_duration as f64 / ttl_bals.max_days_in_year as f64,
    );

    stamp_default(&mut one_acc_out);
}
