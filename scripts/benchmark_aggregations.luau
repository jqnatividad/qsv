-- version 1.1
-- this is the BEGIN block, which is executed once before the main loop
BEGIN {
    -- we initialize the variables here
    total_mean = 0;
    rounded_mean = 0;
    benchmark_data_rowcount=tonumber(qsv_getenv("QSVBM_ROWCOUNT"));

    -- and setup helper functions
    -- this one is to round a number to a given number of decimal places
    function round(num, numDecimalPlaces)
        return string.format("%." .. (numDecimalPlaces or 0) .. "f", num)
    end
}!

-- this is the main loop, which is executed for each row
-- we round the mean to 3 decimal places and return it as a string
-- we do so as we display the rounded mean in the output
-- and computing recs/sec with the unrounded mean would show slightly 
-- different results for the same rounded_mean
rounded_mean = round(mean, 3);
work_var = benchmark_data_rowcount / rounded_mean; 
total_mean = total_mean + rounded_mean; 

recs_per_sec=string.format("%.0f",work_var);
return recs_per_sec;

-- this is the end block, which is executed once after the main loop
END {
    -- we return the total_mean here
    -- note that this value is returned to stderr
    return round(total_mean, 3);
}!
