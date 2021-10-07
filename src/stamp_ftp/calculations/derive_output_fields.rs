use super::*;

pub fn stamp_adjs(
    one_acc_out: &mut OneAccountView,
    static_params: &StaticParams,
    derived_fields: &DerivedFields,
) -> (f32, f32) {
    let mut fix_adjs = DEFAULT_FLOAT;
    let mut var_adjs = DEFAULT_FLOAT;
    let fix_count = static_params.config_params.fixed_adj_count() as usize;

    for _ in 0..fix_count {
        for index in 0..derived_fields.fix_adjs.len() {
            let adj_key = AdjKey::new(one_acc_out.val_dt, derived_fields.fix_adjs[index]);
            one_acc_out.adj_codes.push(derived_fields.fix_adjs[index]);
            if let Some(adj_rate) = static_params.adj_rates.adjs.get(&adj_key) {
                one_acc_out.adj_rates.push(*adj_rate as f32);
                fix_adjs += *adj_rate;
            } else {
                one_acc_out.adj_rates.push(DEFAULT_FLOAT as f32);
                log_debug!(
                    static_params.log,
                    "Adj rates not found for account: {}, adj id: {}, date: {}.",
                    one_acc_out.adj_codes[index],
                    one_acc_out.val_dt,
                    one_acc_out.account_id
                );
            }
        }
        one_acc_out.adj_codes.push(DEFAULT_INT as i32);
        one_acc_out.adj_rates.push(DEFAULT_FLOAT as f32);
    }

    for _ in 0..static_params.config_params.var_adj_count() as usize {
        for index in 0..derived_fields.var_adjs.len() {
            let adj_key = AdjKey::new(one_acc_out.val_dt, derived_fields.var_adjs[index]);
            one_acc_out.adj_codes.push(derived_fields.var_adjs[index]);
            if let Some(adj_rate) = static_params.adj_rates.adjs.get(&adj_key) {
                one_acc_out.adj_rates.push(*adj_rate as f32);
                var_adjs += *adj_rate;
            } else {
                one_acc_out.adj_rates.push(DEFAULT_FLOAT as f32);
                log_debug!(
                    static_params.log,
                    "Adj rates not found for account: {}, adj id: {}, date: {}..",
                    one_acc_out.adj_codes[fix_count + index],
                    one_acc_out.val_dt,
                    one_acc_out.account_id
                );
            }
        }
        one_acc_out.adj_codes.push(DEFAULT_INT as i32);
        one_acc_out.adj_rates.push(DEFAULT_FLOAT as f32);
    }

    (fix_adjs as f32, var_adjs as f32)
}

pub fn calc_int(prin_amt: f64, rate: f32, time: f64) -> f64 {
    prin_amt * rate as f64 * time / 100.0
}

pub fn get_avg_bal(
    one_acc_out: &mut OneAccountView,
    static_params: &StaticParams,
    dyn_params: &DynamicParams,
) -> f64 {
    if let Some(bals) = dyn_params.avg_bal.avg_bal.get(&one_acc_out.account_id) {
        bals.bal
    } else {
        log_debug!(
            static_params.log,
            "Average Balance not found for account: {}.",
            one_acc_out.account_id
        );
        DEFAULT_FLOAT
    }
}
