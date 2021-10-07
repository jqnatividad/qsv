-- be sure to have the `date.lua` file 
-- from https://github.com/Tieske/date
local date = require "date"

-- replace "Created Date" column with the name
-- of the column in your CSV
local t_date = date(col["Created Date"])
local t_month = t_date:getmonth()
local t_year = t_date:getisoyear()
return t_year .. ' Q' .. math.ceil(t_month / 3)
