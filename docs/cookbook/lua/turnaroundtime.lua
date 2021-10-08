-- be sure to have the `date.lua` file 
-- from https://github.com/Tieske/date
local date = require "date"

-- replace "Created Date"/"Closed Date" columns
-- with the name of the columns in your CSV
if (string.len(col["Created Date"]) == 0 or string.len(col["Closed Date"]) == 0) then
    return
end

local t_sdate = date(col["Created Date"])
local t_edate = date(col["Closed Date"])
local t_diff = date.diff(t_edate, t_sdate)
return t_diff:spandays()
