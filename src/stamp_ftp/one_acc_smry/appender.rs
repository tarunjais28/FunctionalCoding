use super::*;
use crate::configuration_parameters::ConfigurationParameters;

pub fn append_input_fields(
    one_acc_out: &mut OneAccountView,
    account: &AccountWithCFs,
    keys: &AccFieldNames,
    config_params: &ConfigurationParameters,
) {
    one_acc_out.as_on_month = timestamp(*config_params.to_date());
    one_acc_out.account_id = account
        .get_string_for_key(&keys.account_id)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.currency = account
        .get_string_for_key(&keys.currency)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.balance_ccy = account
        .get_f64_for_key(&keys.balance_ccy)
        .unwrap_or_default();
    one_acc_out.int_rate = account
        .get_f32_for_key(&keys.int_rate)
        .unwrap_or(DEFAULT_FLOAT as f32);
    one_acc_out.rate_flag = account
        .get_string_for_key(&keys.rate_flag)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.val_dt = account
        .get_i64_for_key(&keys.val_dt)
        .unwrap_or_else(|_| timestamp(*DEFAULT_DATE));
    one_acc_out.open_dt = account
        .get_i64_for_key(&keys.open_dt)
        .unwrap_or_else(|_| timestamp(*DEFAULT_DATE));
    one_acc_out.mat_dt = account
        .get_i64_for_key(&keys.mat_dt)
        .unwrap_or_else(|_| timestamp(*DEFAULT_DATE));
    one_acc_out.lst_repricing_dt = account
        .get_i64_for_key(&keys.lst_repricing_dt)
        .unwrap_or_else(|_| timestamp(*DEFAULT_DATE));
    one_acc_out.rep_freq = account
        .get_string_for_key(&keys.rep_freq)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.cust_agg_bal = account
        .get_f64_for_key(&keys.cust_agg_bal)
        .unwrap_or_default();
    one_acc_out.day_count_basis = account
        .get_string_for_key(&keys.day_count_basis)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.a_or_l = account
        .get_string_for_key(&keys.a_or_l)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.dim1 = account
        .get_string_for_key(&keys.dim1)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.dim2 = account
        .get_string_for_key(&keys.dim2)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.dim3 = account
        .get_string_for_key(&keys.dim3)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.dim4 = account
        .get_string_for_key(&keys.dim4)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.customer_id = account
        .get_string_for_key(&keys.customer_id)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.rl1 = account
        .get_string_for_key(&keys.rl1)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.rl2 = account
        .get_string_for_key(&keys.rl2)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.rl3 = account
        .get_string_for_key(&keys.rl3)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.gl_code = if let Ok(cd) = account.get_i64_for_key(&keys.gl_code) {
        get_string_from_i64(cd)
    } else if let Ok(cd) = account.get_string_for_key(&keys.gl_code) {
        cd.to_string()
    } else {
        DEFAULT_STRING.to_string()
    };
    one_acc_out.prod_code = account
        .get_string_for_key(&keys.prod_code)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.div_code = account
        .get_string_for_key(&keys.div_code)
        .unwrap_or(&DEFAULT_STRING.to_string())
        .to_string();
    one_acc_out.mis_code_1 = if let Ok(cd) = account.get_i64_for_key(&keys.mis_code_1) {
        get_string_from_i64(cd)
    } else if let Ok(cd) = account.get_string_for_key(&keys.mis_code_1) {
        cd.to_string()
    } else {
        DEFAULT_STRING.to_string()
    };
    one_acc_out.mis_code_2 = if let Ok(cd) = account.get_i64_for_key(&keys.mis_code_2) {
        get_string_from_i64(cd)
    } else if let Ok(cd) = account.get_string_for_key(&keys.mis_code_2) {
        cd.to_string()
    } else {
        DEFAULT_STRING.to_string()
    };
    one_acc_out.mis_code_3 = if let Ok(cd) = account.get_i64_for_key(&keys.mis_code_3) {
        get_string_from_i64(cd)
    } else if let Ok(cd) = account.get_string_for_key(&keys.mis_code_3) {
        cd.to_string()
    } else {
        DEFAULT_STRING.to_string()
    };
}

pub fn append_rules_based_dates(one_acc_out: &mut OneAccountView, parsed_method: &ParsedMethod) {
    one_acc_out.bc_as_on_rule = timestamp(parsed_method.curve_pick_date);
    one_acc_out.bc_as_on_applied = timestamp(parsed_method.curve_pick_date);
    one_acc_out.tenor_start_date_rule = timestamp(parsed_method.tenor_start_date);
    one_acc_out.tenor_start_date_applied = timestamp(parsed_method.tenor_start_date);
    one_acc_out.tenor_end_date_rule = timestamp(parsed_method.tenor_end_date);
    one_acc_out.tenor_end_date_applied = timestamp(parsed_method.tenor_end_date);
}
