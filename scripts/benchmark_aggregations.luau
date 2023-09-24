-- this is the BEGIN block, which is executed once before the main loop
BEGIN {
    -- we initialize the variables here
    total_mean = 0;
    benchmark_data_rowcount=tonumber(qsv_getenv("QSVBM_ROWCOUNT"));

    -- and setup helper functions
    -- this one is to round a number to a given number of decimal places
    function round(num, numDecimalPlaces)
        return string.format("%." .. (numDecimalPlaces or 0) .. "f", num)
    end
}!

-- this is the main loop, which is executed for each row
work_var = benchmark_data_rowcount / mean; 
total_mean = total_mean + mean; 

recs_per_sec=string.format("%.0f",work_var);
return recs_per_sec;

-- this is the end block, which is executed once after the main loop
END {
    -- we return the total_mean here
    -- note that this value is returned to stderr
    return round(total_mean, 3);
}!
