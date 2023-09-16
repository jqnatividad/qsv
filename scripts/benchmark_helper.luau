-- this is a helper function to add commas to a number
-- we use this to make recs_per_sec more readable
function numWithCommas( n: number ): string
    return tostring(math.floor(n)):reverse():gsub("(%d%d%d)","%1,")
           :gsub(",(%-?)$","%1"):reverse()
end
