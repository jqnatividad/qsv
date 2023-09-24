-- compute Turnaround Time for NYC 311 Benchmark data
local date = require "date"

if (string.len(col["Created Date"]) == 0 or string.len(col["Closed Date"]) == 0) then
    return
end

local t_sdate = date(col["Created Date"])
local t_edate = date(col["Closed Date"])
local t_diff = date.diff(t_edate, t_sdate)
return t_diff:spandays()
