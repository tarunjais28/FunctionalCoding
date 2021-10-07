#!/usr/bin/env bash

FROM_DATE=$"01-01-2019"
TO_DATE=$"31-01-2019"
CONFIG=$"test-bed/config.json"
M_RULE=$"test-bed/loans-method-rules.txt"
BC_RULE=$"test-bed/loans-bc-rules.txt"
ADJ_RULE_FIX=$"test-bed/loans-fix-adj.txt"
ADJ_RULE_VAR=$"test-bed/loans-var-adj.txt"
SPREAD=$"test-bed/loans-spread.txt"
ADJ_RATES=$"test-bed/adj.txt"
OUTPUT=$"test-bed/output/output"
METHOD=$"test-bed/method-req-fields.json"
BAL_SLAB=$"test-bed/loans-bal-slab.txt"
BC_FILE=$"test-bed/BMRates"
LOG_FILE=$"test-bed/output/loans_log.txt"
DIAGNOSTICS_FILE=$"test-bed/output/loans_diag_log.txt"

cargo run --release -- \
    --from-date "01-05-2021" \
    --to-date "31-05-2021" \
    --config-file ${CONFIG} \
    --method-rules-file ${M_RULE} \
    --bc-rule-file ${BC_RULE} \
    --fix-adj-rule-file ${ADJ_RULE_FIX} \
    --var-adj-rule-file ${ADJ_RULE_VAR} \
    --output-file ${OUTPUT} \
    --bc-file ${BC_FILE} \
    --spread-file ${SPREAD} \
    --adj-rates-file ${ADJ_RATES} \
    --method-req-fields-file-path ${METHOD} \
    --default-method 1002 \
    --default-basecurve 1110 \
    --fixed-adjustments-count 3 \
    --var-adjustments-count 3 \
    --log-file ${LOG_FILE} \
    --diagnostics-log-file ${DIAGNOSTICS_FILE} \
    --log-level trace \
    --bal-prec 4 \
    --rate-prec 4 \
    --diagnostics-flag true \
    --ccy INR \
    --is-def-from-date true \
    --bal-slab ${BAL_SLAB}
