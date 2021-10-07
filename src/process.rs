use super::*;

pub fn calculate_ftp(log: &Logger, diag_log: &Logger, config_params: &ConfigurationParameters) {
    let mut tot_cfs = 0;
    let mut tot_rec = 0;
    let mut total_amt_inp = 0.0;
    let mut total_amt_out = 0.0;

    let mut old_acc_map = SpreadReader::new(config_params.spread_file_path());
    let mut one_acc_out = OneAccountView::new();
    // Deriving static parameters which are common throughout program
    let mut static_params: StaticParams = StaticParams::new(log, diag_log, config_params);

    // Reading files in configuration json
    let files_config = get_files(config_params.config_file_path());
    for file in files_config.files {
        let mut output: String = String::new();

        // Deriving fields which are common throughout file
        let mut dyn_params: DynamicParams = DynamicParams::new(config_params, &file);
        let input_file_path =
            get_file_path(file.input_file_path.to_string(), *config_params.to_date());
        let mut input_data = Reader::new_at_path(&file.metadata_file_path, &input_file_path);
        for mut account in input_data.iter() {
            tot_rec += 1;

            // Passing pashthroughs to the stamper output
            append_input_fields(
                &mut one_acc_out,
                &account,
                &dyn_params.input_field_names,
                config_params,
            );

            // Deriving fields which are common throughout records
            let mut derived_fields = DerivedFields::new(
                &one_acc_out.account_id,
                &mut account,
                &static_params,
                &dyn_params,
            );

            // Apply FTP calculation logic
            calc_ftp(
                &mut &mut one_acc_out,
                &mut static_params,
                &mut dyn_params,
                &mut derived_fields,
                &mut old_acc_map,
            );

            tot_cfs += derived_fields.cashflows.len();
            total_amt_inp += one_acc_out.cust_agg_bal;
            total_amt_out += one_acc_out.ftp_amt_ccy;

            // Convert amount in consolidated currency
            ccy_converted(&mut one_acc_out, &mut static_params, &mut dyn_params);

            // Apply round off to amount and rate fields
            one_acc_out.rounded(config_params.rate_prec(), config_params.bal_prec());

            // Copying same fields in stamper output
            one_acc_out.copied();
            output.push_str(&one_acc_out.print());
            one_acc_out.clear();
        }
        // Writing Stamper output
        write_output(&mut static_params, output);
    }

    // Writing health check report
    let health_report = HealthReport::new(
        tot_rec,
        tot_rec,
        0,
        total_amt_inp,
        total_amt_out,
        tot_cfs as i64,
    );
    health_report.gen_health_rpt(config_params.output_file_path());
}
