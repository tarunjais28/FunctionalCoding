use self::acc_level::stamp_acc_level;
pub use self::bm_rates::*;
use self::cf_level::stamp_cf_level;
use self::default::stamp_default;
use self::derive_output_fields::*;
use self::lock::stamp_lock;
use self::margin::stamp_margin;
use self::rep_lock::stamp_rep_lock;
use super::*;
use std::path::Path;

mod acc_level;
mod bm_rates;
mod cf_level;
mod default;
mod derive_output_fields;
mod lock;
mod margin;
mod rep_lock;

pub fn calc_ftp(
    mut one_acc_out: &mut OneAccountView,
    mut static_params: &mut StaticParams,
    mut dyn_params: &mut DynamicParams,
    mut derived_fields: &mut DerivedFields,
    old_acc_map: &mut OldAccountMap,
) {
    append_rules_based_dates(one_acc_out, &derived_fields.parsed_method);
    one_acc_out.ftp_method = get_method_name(derived_fields.method_id).to_string();

    match derived_fields.method_id {
        1001 => stamp_acc_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &derived_fields,
        ),
        1002 => stamp_acc_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &derived_fields,
        ),
        1003 => stamp_acc_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &derived_fields,
        ),
        1011 => stamp_cf_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &mut derived_fields,
        ),
        1012 => stamp_cf_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &mut derived_fields,
        ),
        1021 => stamp_cf_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &mut derived_fields,
        ),
        1022 => stamp_acc_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &derived_fields,
        ),
        1023 => stamp_acc_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &derived_fields,
        ),
        1031 => {
            if let Some(rates) = old_acc_map.get(&one_acc_out.account_id) {
                if rates.method == derived_fields.method_id || rates.method == 0 {
                    stamp_lock(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        derived_fields,
                        rates,
                    );
                } else {
                    stamp_cf_level(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        &mut derived_fields,
                    );
                }
                stamp_cf_level(
                    &mut one_acc_out,
                    &mut static_params,
                    &mut dyn_params,
                    &mut derived_fields,
                );
            }
        }
        1032 => {
            if let Some(rates) = old_acc_map.get(&one_acc_out.account_id) {
                if rates.method == derived_fields.method_id || rates.method == 0 {
                    stamp_lock(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        derived_fields,
                        rates,
                    );
                } else {
                    stamp_acc_level(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        &derived_fields,
                    );
                }
                stamp_acc_level(
                    &mut one_acc_out,
                    &mut static_params,
                    &mut dyn_params,
                    &derived_fields,
                );
            }
        }
        1033 => {
            if let Some(rates) = old_acc_map.get(&one_acc_out.account_id) {
                if rates.method == derived_fields.method_id || rates.method == 0 {
                    stamp_lock(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        derived_fields,
                        rates,
                    );
                } else {
                    stamp_acc_level(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        &derived_fields,
                    );
                }
                stamp_acc_level(
                    &mut one_acc_out,
                    &mut static_params,
                    &mut dyn_params,
                    &derived_fields,
                );
            }
        }
        1034 => {
            if let Some(rates) = old_acc_map.get(&one_acc_out.account_id) {
                if rates.method == derived_fields.method_id || rates.method == 0 {
                    stamp_lock(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        derived_fields,
                        rates,
                    );
                } else {
                    stamp_acc_level(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        &derived_fields,
                    );
                }
                stamp_acc_level(
                    &mut one_acc_out,
                    &mut static_params,
                    &mut dyn_params,
                    &derived_fields,
                );
            }
        }
        1036 => {
            if let Some(rates) = old_acc_map.get(&one_acc_out.account_id) {
                if rates.method == derived_fields.method_id || rates.method == 0 {
                    stamp_rep_lock(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        derived_fields,
                        rates,
                    );
                } else {
                    stamp_acc_level(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        &derived_fields,
                    );
                }
                stamp_acc_level(
                    &mut one_acc_out,
                    &mut static_params,
                    &mut dyn_params,
                    &derived_fields,
                );
            }
        }
        1041 => stamp_margin(&mut one_acc_out, &mut static_params, &mut dyn_params),
        _ => stamp_default(&mut one_acc_out),
    }
}
