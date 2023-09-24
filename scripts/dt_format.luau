local date = require "date";

createdate = date(col["Created Date"]);

dayofweek = createdate:fmt('%a');
hourofday = createdate:fmt('%H');
weeknumber = createdate:fmt('%W');
return {dayofweek, hourofday, weeknumber};
