-- version 1.1.1
-- this luau script is used by benchmarks.sh to compute records per second (recs_per_sec)
-- for each benchmark. It does this by retrieving the rowcount from the environment var 
-- QSVBM_ROWCOUNT (which is set by benchmarks.sh by running "qsv count") and dividing it 
-- by the mean value of the elapsed time for for each benchmark's runs (default: 3 runs).
-- It also computes the total mean in the END block, which is returned to stderr.
-- The total mean is reported by benchmarks.sh at the end of each run. 
-- The total mean gives an idea of how long the benchmark suite took to run, 
-- regardless of the number of runs per benchmark.

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

-- this is the main loop, which is executed for each row in the CSV
-- "mean" is the name of one of the columns in the CSV. That's right,
-- we can "magically" access the columns in the CSV directly as luau variables!
-- for each row, the luau var "mean" is set to the current value of the column "mean"
-- and the column "recs_per_sec" for that row is "map"ped to the value returned by the main loop
--
-- we round the mean to 3 decimal places and return it as a string
-- we do so as we display the rounded mean in the output
-- and computing recs/sec with the unrounded mean would show slightly 
-- different results for the same rounded_mean
rounded_mean = round(mean, 3);
-- for clarity, instead of this "magic" parsing of CSV data,
-- we could have also written:
--    rounded_mean = round(col.mean , 3); 
-- or
--    rounded_mean = round(col['mean'] , 3);
-- this is especially useful if the column has embedded spaces or special characters
-- or if the column name is a luau keyword (like "end" or "function")

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
