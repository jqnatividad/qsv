---------------------------------------------------------------------------------------
-- Module for date and time calculations
--
-- Version 2.2
-- Copyright (C) 2005-2006, by Jas Latrix (jastejada@yahoo.com)
-- Copyright (C) 2013-2021, by Thijs Schreijer
-- Licensed under MIT, http://opensource.org/licenses/MIT

--[[ CONSTANTS ]]--
  local HOURPERDAY  = 24
  local MINPERHOUR  = 60
  local MINPERDAY    = 1440  -- 24*60
  local SECPERMIN   = 60
  local SECPERHOUR  = 3600  -- 60*60
  local SECPERDAY   = 86400 -- 24*60*60
  local TICKSPERSEC = 1000000
  local TICKSPERDAY = 86400000000
  local TICKSPERHOUR = 3600000000
  local TICKSPERMIN = 60000000
  local DAYNUM_MAX =  365242500 -- Sat Jan 01 1000000 00:00:00
  local DAYNUM_MIN = -365242500 -- Mon Jan 01 1000000 BCE 00:00:00
  local DAYNUM_DEF =  0 -- Mon Jan 01 0001 00:00:00
  local _;
--[[ GLOBAL SETTINGS ]]--
  local centuryflip = 0 -- year >= centuryflip == 1900, < centuryflip == 2000
--[[ LOCAL ARE FASTER ]]--
  local type     = type
  local pairs    = pairs
  local error    = error
  local assert   = assert
  local tonumber = tonumber
  local tostring = tostring
  local string   = string
  local math     = math
  local os       = os
  local unpack   = unpack or table.unpack
  local setmetatable = setmetatable
  local getmetatable = getmetatable
--[[ EXTRA FUNCTIONS ]]--
  local fmt  = string.format
  local lwr  = string.lower
  local rep  = string.rep
  local len  = string.len  -- luacheck: ignore
  local sub  = string.sub
  local gsub = string.gsub
  local gmatch = string.gmatch or string.gfind
  local find = string.find
  local ostime = os.time
  local osdate = os.date
  local floor = math.floor
  local ceil  = math.ceil
  local abs   = math.abs
  -- removes the decimal part of a number
  local function fix(n) n = tonumber(n) return n and ((n > 0 and floor or ceil)(n)) end
  -- returns the modulo n % d;
  local function mod(n,d) return n - d*floor(n/d) end
  -- is `str` in string list `tbl`, `ml` is the minimun len
  local function inlist(str, tbl, ml, tn)
    local sl = len(str)
    if sl < (ml or 0) then return nil end
    str = lwr(str)
    for k, v in pairs(tbl) do
      if str == lwr(sub(v, 1, sl)) then
        if tn then tn[0] = k end
        return k
      end
    end
  end
  local function fnil() end
--[[ DATE FUNCTIONS ]]--
  local DATE_EPOCH -- to be set later
  local sl_weekdays = {
    [0]="Sunday",[1]="Monday",[2]="Tuesday",[3]="Wednesday",[4]="Thursday",[5]="Friday",[6]="Saturday",
    [7]="Sun",[8]="Mon",[9]="Tue",[10]="Wed",[11]="Thu",[12]="Fri",[13]="Sat",
  }
  local sl_meridian = {[-1]="AM", [1]="PM"}
  local sl_months = {
    [00]="January", [01]="February", [02]="March",
    [03]="April",   [04]="May",      [05]="June",
    [06]="July",    [07]="August",   [08]="September",
    [09]="October", [10]="November", [11]="December",
    [12]="Jan", [13]="Feb", [14]="Mar",
    [15]="Apr", [16]="May", [17]="Jun",
    [18]="Jul", [19]="Aug", [20]="Sep",
    [21]="Oct", [22]="Nov", [23]="Dec",
  }
  -- added the '.2'  to avoid collision, use `fix` to remove
  local sl_timezone = {
    [000]="utc",    [0.2]="gmt",
    [300]="est",    [240]="edt",
    [360]="cst",  [300.2]="cdt",
    [420]="mst",  [360.2]="mdt",
    [480]="pst",  [420.2]="pdt",
  }
  -- set the day fraction resolution
  local function setticks(t)
    TICKSPERSEC = t;
    TICKSPERDAY = SECPERDAY*TICKSPERSEC
    TICKSPERHOUR= SECPERHOUR*TICKSPERSEC
    TICKSPERMIN = SECPERMIN*TICKSPERSEC
  end
  -- is year y leap year?
  local function isleapyear(y) -- y must be int!
    return (mod(y, 4) == 0 and (mod(y, 100) ~= 0 or mod(y, 400) == 0))
  end
  -- day since year 0
  local function dayfromyear(y) -- y must be int!
    return 365*y + floor(y/4) - floor(y/100) + floor(y/400)
  end
  -- day number from date, month is zero base
  local function makedaynum(y, m, d)
    local mm = mod(mod(m,12) + 10, 12)
    return dayfromyear(y + floor(m/12) - floor(mm/10)) + floor((mm*306 + 5)/10) + d - 307
    --local yy = y + floor(m/12) - floor(mm/10)
    --return dayfromyear(yy) + floor((mm*306 + 5)/10) + (d - 1)
  end
  -- date from day number, month is zero base
  local function breakdaynum(g)
    local g = g + 306
    local y = floor((10000*g + 14780)/3652425)
    local d = g - dayfromyear(y)
    if d < 0 then y = y - 1; d = g - dayfromyear(y) end
    local mi = floor((100*d + 52)/3060)
    return (floor((mi + 2)/12) + y), mod(mi + 2,12), (d - floor((mi*306 + 5)/10) + 1)
  end
  --[[ for floats or int32 Lua Number data type
  local function breakdaynum2(g)
    local g, n = g + 306;
    local n400 = floor(g/DI400Y);n = mod(g,DI400Y);
    local n100 = floor(n/DI100Y);n = mod(n,DI100Y);
    local n004 = floor(n/DI4Y);   n = mod(n,DI4Y);
    local n001 = floor(n/365);   n = mod(n,365);
    local y = (n400*400) + (n100*100) + (n004*4) + n001  - ((n001 == 4 or n100 == 4) and 1 or 0)
    local d = g - dayfromyear(y)
    local mi = floor((100*d + 52)/3060)
    return (floor((mi + 2)/12) + y), mod(mi + 2,12), (d - floor((mi*306 + 5)/10) + 1)
  end
  ]]
  -- day fraction from time
  local function makedayfrc(h,r,s,t)
    return ((h*60 + r)*60 + s)*TICKSPERSEC + t
  end
  -- time from day fraction
  local function breakdayfrc(df)
    return
      mod(floor(df/TICKSPERHOUR),HOURPERDAY),
      mod(floor(df/TICKSPERMIN ),MINPERHOUR),
      mod(floor(df/TICKSPERSEC ),SECPERMIN),
      mod(df,TICKSPERSEC)
  end
  -- weekday sunday = 0, monday = 1 ...
  local function weekday(dn) return mod(dn + 1, 7) end
  -- yearday 0 based ...
  local function yearday(dn)
     return dn - dayfromyear((breakdaynum(dn))-1)
  end
  -- parse v as a month
  local function getmontharg(v)
    local m = tonumber(v);
    return (m and fix(m - 1)) or inlist(tostring(v) or "", sl_months, 2)
  end
  -- get daynum of isoweek one of year y
  local function isow1(y)
    local f = makedaynum(y, 0, 4) -- get the date for the 4-Jan of year `y`
    local d = weekday(f)
    d = d == 0 and 7 or d -- get the ISO day number, 1 == Monday, 7 == Sunday
    return f + (1 - d)
  end
  local function isowy(dn)
    local w1;
    local y = (breakdaynum(dn))
    if dn >= makedaynum(y, 11, 29) then
      w1 = isow1(y + 1);
      if dn < w1 then
        w1 = isow1(y);
      else
          y = y + 1;
      end
    else
      w1 = isow1(y);
      if dn < w1 then
        w1 = isow1(y-1)
        y = y - 1
      end
    end
    return floor((dn-w1)/7)+1, y
  end
  local function isoy(dn)
    local y = (breakdaynum(dn))
    return y + (((dn >= makedaynum(y, 11, 29)) and (dn >= isow1(y + 1))) and 1 or (dn < isow1(y) and -1 or 0))
  end
  local function makedaynum_isoywd(y,w,d)
    return isow1(y) + 7*w + d - 8 -- simplified: isow1(y) + ((w-1)*7) + (d-1)
  end
--[[ THE DATE MODULE ]]--
  local fmtstr  = "%x %X";
--#if not DATE_OBJECT_AFX then
  local date = {}
  setmetatable(date, date)
-- Version:  VMMMRRRR; V-Major, M-Minor, R-Revision;  e.g. 5.45.321 == 50450321
  do
    local major = 2
    local minor = 2
    local revision = 0
    date.version = major * 10000000 + minor * 10000 + revision
  end
--#end -- not DATE_OBJECT_AFX
--[[ THE DATE OBJECT ]]--
  local dobj = {}
  dobj.__index = dobj
  dobj.__metatable = dobj
  -- shout invalid arg
  local function date_error_arg() return error("invalid argument(s)",0) end
  -- create new date object
  local function date_new(dn, df)
    return setmetatable({daynum=dn, dayfrc=df}, dobj)
  end

--#if not NO_LOCAL_TIME_SUPPORT then
  -- magic year table
  local date_epoch, yt;
  local function getequivyear(y)
    assert(not yt)
    yt = {}
    local de = date_epoch:copy()
    local dw, dy
    for _ = 0, 3000 do
      de:setyear(de:getyear() + 1, 1, 1)
      dy = de:getyear()
      dw = de:getweekday() * (isleapyear(dy) and  -1 or 1)
      if not yt[dw] then yt[dw] = dy end  --print(de)
      if yt[1] and yt[2] and yt[3] and yt[4] and yt[5] and yt[6] and yt[7] and yt[-1] and yt[-2] and yt[-3] and yt[-4] and yt[-5] and yt[-6] and yt[-7] then
        getequivyear = function(y)  return yt[ (weekday(makedaynum(y, 0, 1)) + 1) * (isleapyear(y) and  -1 or 1) ]  end
        return getequivyear(y)
      end
    end
  end
  -- TimeValue from date and time
  local function totv(y,m,d,h,r,s)
    return (makedaynum(y, m, d) - DATE_EPOCH) * SECPERDAY  + ((h*60 + r)*60 + s)
  end
  -- TimeValue from TimeTable
  local function tmtotv(tm)
    return tm and totv(tm.year, tm.month - 1, tm.day, tm.hour, tm.min, tm.sec)
  end
  -- Returns the bias in seconds of utc time daynum and dayfrc
  local function getbiasutc2(self)
    local y,m,d = breakdaynum(self.daynum)
    local h,r,s = breakdayfrc(self.dayfrc)
    local tvu = totv(y,m,d,h,r,s) -- get the utc TimeValue of date and time
    local tml = osdate("*t", tvu) -- get the local TimeTable of tvu
    if (not tml) or (tml.year > (y+1) or tml.year < (y-1)) then -- failed try the magic
      y = getequivyear(y)
      tvu = totv(y,m,d,h,r,s)
      tml = osdate("*t", tvu)
    end
    local tvl = tmtotv(tml)
    if tvu and tvl then
      return tvu - tvl, tvu, tvl
    else
      return error("failed to get bias from utc time")
    end
  end
  -- Returns the bias in seconds of local time daynum and dayfrc
  local function getbiasloc2(daynum, dayfrc)
    local tvu
    -- extract date and time
    local y,m,d = breakdaynum(daynum)
    local h,r,s = breakdayfrc(dayfrc)
    -- get equivalent TimeTable
    local tml = {year=y, month=m+1, day=d, hour=h, min=r, sec=s}
    -- get equivalent TimeValue
    local tvl = tmtotv(tml)

    local function chkutc()
      tml.isdst =  nil; local tvug = ostime(tml) if tvug and (tvl == tmtotv(osdate("*t", tvug))) then tvu = tvug return end
      tml.isdst = true; local tvud = ostime(tml) if tvud and (tvl == tmtotv(osdate("*t", tvud))) then tvu = tvud return end
      tvu = tvud or tvug
    end
    chkutc()
    if not tvu then
      tml.year = getequivyear(y)
      tvl = tmtotv(tml)
      chkutc()
    end
    return ((tvu and tvl) and (tvu - tvl)) or error("failed to get bias from local time"), tvu, tvl
  end
--#end -- not NO_LOCAL_TIME_SUPPORT

--#if not DATE_OBJECT_AFX then
  -- the date parser
  local strwalker = {} -- ^Lua regular expression is not as powerful as Perl$
  strwalker.__index = strwalker
  local function newstrwalker(s)return setmetatable({s=s, i=1, e=1, c=len(s)}, strwalker) end
  function strwalker:aimchr() return "\n" .. self.s .. "\n" .. rep(".",self.e-1) .. "^" end
  function strwalker:finish() return self.i > self.c  end
  function strwalker:back()  self.i = self.e return self  end
  function strwalker:restart() self.i, self.e = 1, 1 return self end
  function strwalker:match(s)  return (find(self.s, s, self.i)) end
  function strwalker:__call(s, f)-- print("strwalker:__call "..s..self:aimchr())
    local is, ie; is, ie, self[1], self[2], self[3], self[4], self[5] = find(self.s, s, self.i)
    if is then self.e, self.i = self.i, 1+ie; if f then f(unpack(self)) end return self end
  end
   local function date_parse(str)
    local y,m,d, h,r,s,  z,  w,u, j,  e,  x,c,  dn,df
    local sw = newstrwalker(gsub(gsub(str, "(%b())", ""),"^(%s*)","")) -- remove comment, trim leading space
    --local function error_out() print(y,m,d,h,r,s) end
    local function error_dup(q) --[[error_out()]] error("duplicate value: " .. (q or "") .. sw:aimchr()) end
    local function error_syn(q) --[[error_out()]] error("syntax error: " .. (q or "") .. sw:aimchr()) end
    local function error_inv(q) --[[error_out()]] error("invalid date: " .. (q or "") .. sw:aimchr()) end
    local function sety(q) y = y and error_dup() or tonumber(q); end
    local function setm(q) m = (m or w or j) and error_dup(m or w or j) or tonumber(q) end
    local function setd(q) d = d and error_dup() or tonumber(q) end
    local function seth(q) h = h and error_dup() or tonumber(q) end
    local function setr(q) r = r and error_dup() or tonumber(q) end
    local function sets(q) s = s and error_dup() or tonumber(q) end
    local function adds(q) s = s + tonumber(q) end
    local function setj(q) j = (m or w or j) and error_dup() or tonumber(q); end
    local function setz(q) z = (z ~= 0 and z) and error_dup() or q end
    local function setzn(zs,zn) zn = tonumber(zn); setz( ((zn<24) and (zn*60) or (mod(zn,100) + floor(zn/100) * 60))*( zs=='+' and -1 or 1) ) end
    local function setzc(zs,zh,zm) setz( ((tonumber(zh)*60) + tonumber(zm))*( zs=='+' and -1 or 1) ) end

    if not (sw("^(%d%d%d%d)",sety) and (sw("^(%-?)(%d%d)%1(%d%d)",function(_,a,b) setm(tonumber(a)); setd(tonumber(b)) end) or sw("^(%-?)[Ww](%d%d)%1(%d?)",function(_,a,b) w, u = tonumber(a), tonumber(b or 1) end) or sw("^%-?(%d%d%d)",setj) or sw("^%-?(%d%d)",function(a) setm(a);setd(1) end))
    and ((sw("^%s*[Tt]?(%d%d):?",seth) and sw("^(%d%d):?",setr) and sw("^(%d%d)",sets) and sw("^(%.%d+)",adds))
      or sw:finish() or (sw"^%s*$" or sw"^%s*[Zz]%s*$" or sw("^%s-([%+%-])(%d%d):?(%d%d)%s*$",setzc) or sw("^%s*([%+%-])(%d%d)%s*$",setzn))
      )  )
    then --print(y,m,d,h,r,s,z,w,u,j)
    sw:restart(); y,m,d,h,r,s,z,w,u,j = nil,nil,nil,nil,nil,nil,nil,nil,nil,nil
      repeat -- print(sw:aimchr())
        if sw("^[tT:]?%s*(%d%d?):",seth) then --print("$Time")
          _ = sw("^%s*(%d%d?)",setr) and sw("^%s*:%s*(%d%d?)",sets) and sw("^(%.%d+)",adds)
        elseif sw("^(%d+)[/\\%s,-]?%s*") then --print("$Digits")
          x, c = tonumber(sw[1]), len(sw[1])
          if (x >= 70) or (m and d and (not y)) or (c > 3) then
            sety( x + ((x >= 100 or c>3) and 0 or x<centuryflip and 2000 or 1900) )
          else
            if m then setd(x) else m = x end
          end
        elseif sw("^(%a+)[/\\%s,-]?%s*") then --print("$Words")
          x = sw[1]
          if inlist(x, sl_months,   2, sw) then
            if m and (not d) and (not y) then d, m = m, false end
            setm(mod(sw[0],12)+1)
          elseif inlist(x, sl_timezone, 2, sw) then
            c = fix(sw[0]) -- ignore gmt and utc
            if c ~= 0 then setz(c, x) end
          elseif not inlist(x, sl_weekdays, 2, sw) then
            sw:back()
            -- am pm bce ad ce bc
            if sw("^([bB])%s*(%.?)%s*[Cc]%s*(%2)%s*[Ee]%s*(%2)%s*") or sw("^([bB])%s*(%.?)%s*[Cc]%s*(%2)%s*") then
              e = e and error_dup() or -1
            elseif sw("^([aA])%s*(%.?)%s*[Dd]%s*(%2)%s*") or sw("^([cC])%s*(%.?)%s*[Ee]%s*(%2)%s*") then
              e = e and error_dup() or 1
            elseif sw("^([PApa])%s*(%.?)%s*[Mm]?%s*(%2)%s*") then
              x = lwr(sw[1]) -- there should be hour and it must be correct
              if (not h) or (h > 12) or (h < 0) then return error_inv() end
              if x == 'a' and h == 12 then h = 0 end -- am
              if x == 'p' and h ~= 12 then h = h + 12 end -- pm
            else error_syn() end
          end
        elseif not(sw("^([+-])(%d%d?):(%d%d)",setzc) or sw("^([+-])(%d+)",setzn) or sw("^[Zz]%s*$")) then -- sw{"([+-])",{"(%d%d?):(%d%d)","(%d+)"}}
          error_syn("?")
        end
      sw("^%s*")  until sw:finish()
    --else print("$Iso(Date|Time|Zone)")
    end
    -- if date is given, it must be complete year, month & day
    if (not y and not h) or ((m and not d) or (d and not m)) or ((m and w) or (m and j) or (j and w)) then return error_inv("!") end
    -- fix month
    if m then m = m - 1 end
    -- fix year if we are on BCE
    if e and e < 0 and y > 0 then y = 1 - y end
    --  create date object
    dn = (y and ((w and makedaynum_isoywd(y,w,u)) or (j and makedaynum(y, 0, j)) or makedaynum(y, m, d))) or DAYNUM_DEF
    df = makedayfrc(h or 0, r or 0, s or 0, 0) + ((z or 0)*TICKSPERMIN)
    --print("Zone",h,r,s,z,m,d,y,df)
    return date_new(dn, df) -- no need to :normalize();
   end
  local function date_fromtable(v)
    local y, m, d = fix(v.year), getmontharg(v.month), fix(v.day)
    local h, r, s, t = tonumber(v.hour), tonumber(v.min), tonumber(v.sec), tonumber(v.ticks)
    -- atleast there is time or complete date
    if (y or m or d) and (not(y and m and d)) then return error("incomplete table")  end
    return (y or h or r or s or t) and date_new(y and makedaynum(y, m, d) or DAYNUM_DEF, makedayfrc(h or 0, r or 0, s or 0, t or 0))
  end
  local tmap = {
    ['number'] = function(v) return date_epoch:copy():addseconds(v) end,
    ['string'] = function(v) return date_parse(v) end,
    ['boolean']= function(v) return date_fromtable(osdate(v and "!*t" or "*t")) end,
    ['table']  = function(v) local ref = getmetatable(v) == dobj; return ref and v or date_fromtable(v), ref end
  }
  local function date_getdobj(v)
    local o, r = (tmap[type(v)] or fnil)(v);
    return (o and o:normalize() or error"invalid date time value"), r -- if r is true then o is a reference to a date obj
  end
--#end -- not DATE_OBJECT_AFX
  local function date_from(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    local y, m, d = fix(arg1), getmontharg(arg2), fix(arg3)
    local h, r, s, t = tonumber(arg4 or 0), tonumber(arg5 or 0), tonumber(arg6 or 0), tonumber(arg7 or 0)
    if y and m and d and h and r and s and t then
      return date_new(makedaynum(y, m, d), makedayfrc(h, r, s, t)):normalize()
    else
      return date_error_arg()
    end
  end

 --[[ THE DATE OBJECT METHODS ]]--
  function dobj:normalize()
    local dn, df = fix(self.daynum), self.dayfrc
    self.daynum, self.dayfrc = dn + floor(df/TICKSPERDAY), mod(df, TICKSPERDAY)
    return (dn >= DAYNUM_MIN and dn <= DAYNUM_MAX) and self or error("date beyond imposed limits:"..self)
  end

  function dobj:getdate()  local y, m, d = breakdaynum(self.daynum) return y, m+1, d end
  function dobj:gettime()  return breakdayfrc(self.dayfrc) end

  function dobj:getclockhour() local h = self:gethours() return h>12 and mod(h,12) or (h==0 and 12 or h) end

  function dobj:getyearday() return yearday(self.daynum) + 1 end
  function dobj:getweekday() return weekday(self.daynum) + 1 end   -- in lua weekday is sunday = 1, monday = 2 ...

  function dobj:getyear()   local r,_,_ = breakdaynum(self.daynum)  return r end
  function dobj:getmonth() local _,r,_ = breakdaynum(self.daynum)  return r+1 end-- in lua month is 1 base
  function dobj:getday()   local _,_,r = breakdaynum(self.daynum)  return r end
  function dobj:gethours()  return mod(floor(self.dayfrc/TICKSPERHOUR),HOURPERDAY) end
  function dobj:getminutes()  return mod(floor(self.dayfrc/TICKSPERMIN), MINPERHOUR) end
  function dobj:getseconds()  return mod(floor(self.dayfrc/TICKSPERSEC ),SECPERMIN)  end
  function dobj:getfracsec()  return mod(floor(self.dayfrc/TICKSPERSEC ),SECPERMIN)+(mod(self.dayfrc,TICKSPERSEC)/TICKSPERSEC) end
  function dobj:getticks(u)  local x = mod(self.dayfrc,TICKSPERSEC) return u and ((x*u)/TICKSPERSEC) or x  end

  function dobj:getweeknumber(wdb)
    local wd, yd = weekday(self.daynum), yearday(self.daynum)
    if wdb then
      wdb = tonumber(wdb)
      if wdb then
        wd = mod(wd-(wdb-1),7)-- shift the week day base
      else
        return date_error_arg()
      end
    end
    return (yd < wd and 0) or (floor(yd/7) + ((mod(yd, 7)>=wd) and 1 or 0))
  end

  function dobj:getisoweekday() return mod(weekday(self.daynum)-1,7)+1 end   -- sunday = 7, monday = 1 ...
  function dobj:getisoweeknumber() return (isowy(self.daynum)) end
  function dobj:getisoyear() return isoy(self.daynum)  end
  function dobj:getisodate()
    local w, y = isowy(self.daynum)
    return y, w, self:getisoweekday()
  end
  function dobj:setisoyear(y, w, d)
    local cy, cw, cd = self:getisodate()
    if y then cy = fix(tonumber(y))end
    if w then cw = fix(tonumber(w))end
    if d then cd = fix(tonumber(d))end
    if cy and cw and cd then
      self.daynum = makedaynum_isoywd(cy, cw, cd)
      return self:normalize()
    else
      return date_error_arg()
    end
  end

  function dobj:setisoweekday(d)    return self:setisoyear(nil, nil, d) end
  function dobj:setisoweeknumber(w,d)  return self:setisoyear(nil, w, d)  end

  function dobj:setyear(y, m, d)
    local cy, cm, cd = breakdaynum(self.daynum)
    if y then cy = fix(tonumber(y))end
    if m then cm = getmontharg(m)  end
    if d then cd = fix(tonumber(d))end
    if cy and cm and cd then
      self.daynum  = makedaynum(cy, cm, cd)
      return self:normalize()
    else
      return date_error_arg()
    end
  end

  function dobj:setmonth(m, d)return self:setyear(nil, m, d) end
  function dobj:setday(d)    return self:setyear(nil, nil, d) end

  function dobj:sethours(h, m, s, t)
    local ch,cm,cs,ck = breakdayfrc(self.dayfrc)
    ch, cm, cs, ck = tonumber(h or ch), tonumber(m or cm), tonumber(s or cs), tonumber(t or ck)
    if ch and cm and cs and ck then
      self.dayfrc = makedayfrc(ch, cm, cs, ck)
      return self:normalize()
    else
      return date_error_arg()
    end
  end

  function dobj:setminutes(m,s,t)  return self:sethours(nil,   m,   s, t) end
  function dobj:setseconds(s, t)  return self:sethours(nil, nil,   s, t) end
  function dobj:setticks(t)    return self:sethours(nil, nil, nil, t) end

  function dobj:spanticks()  return (self.daynum*TICKSPERDAY + self.dayfrc) end
  function dobj:spanseconds()  return (self.daynum*TICKSPERDAY + self.dayfrc)/TICKSPERSEC  end
  function dobj:spanminutes()  return (self.daynum*TICKSPERDAY + self.dayfrc)/TICKSPERMIN  end
  function dobj:spanhours()  return (self.daynum*TICKSPERDAY + self.dayfrc)/TICKSPERHOUR end
  function dobj:spandays()  return (self.daynum*TICKSPERDAY + self.dayfrc)/TICKSPERDAY  end

  function dobj:addyears(y, m, d)
    local cy, cm, cd = breakdaynum(self.daynum)
    if y then y = fix(tonumber(y))else y = 0 end
    if m then m = fix(tonumber(m))else m = 0 end
    if d then d = fix(tonumber(d))else d = 0 end
    if y and m and d then
      self.daynum  = makedaynum(cy+y, cm+m, cd+d)
      return self:normalize()
    else
      return date_error_arg()
    end
  end

  function dobj:addmonths(m, d)
    return self:addyears(nil, m, d)
  end

  local function dobj_adddayfrc(self,n,pt,pd)
    n = tonumber(n)
    if n then
      local x = floor(n/pd);
      self.daynum = self.daynum + x;
      self.dayfrc = self.dayfrc + (n-x*pd)*pt;
      return self:normalize()
    else
      return date_error_arg()
    end
  end
  function dobj:adddays(n)  return dobj_adddayfrc(self,n,TICKSPERDAY,1) end
  function dobj:addhours(n)  return dobj_adddayfrc(self,n,TICKSPERHOUR,HOURPERDAY) end
  function dobj:addminutes(n)  return dobj_adddayfrc(self,n,TICKSPERMIN,MINPERDAY)  end
  function dobj:addseconds(n)  return dobj_adddayfrc(self,n,TICKSPERSEC,SECPERDAY)  end
  function dobj:addticks(n)  return dobj_adddayfrc(self,n,1,TICKSPERDAY) end
  local tvspec = {
    -- Abbreviated weekday name (Sun)
    ['%a']=function(self) return sl_weekdays[weekday(self.daynum) + 7] end,
    -- Full weekday name (Sunday)
    ['%A']=function(self) return sl_weekdays[weekday(self.daynum)] end,
    -- Abbreviated month name (Dec)
    ['%b']=function(self) return sl_months[self:getmonth() - 1 + 12] end,
    -- Full month name (December)
    ['%B']=function(self) return sl_months[self:getmonth() - 1] end,
    -- Year/100 (19, 20, 30)
    ['%C']=function(self) return fmt("%.2d", fix(self:getyear()/100)) end,
    -- The day of the month as a number (range 1 - 31)
    ['%d']=function(self) return fmt("%.2d", self:getday())  end,
    -- year for ISO 8601 week, from 00 (79)
    ['%g']=function(self) return fmt("%.2d", mod(self:getisoyear() ,100)) end,
    -- year for ISO 8601 week, from 0000 (1979)
    ['%G']=function(self) return fmt("%.4d", self:getisoyear()) end,
    -- same as %b
    ['%h']=function(self) return self:fmt0("%b") end,
    -- hour of the 24-hour day, from 00 (06)
    ['%H']=function(self) return fmt("%.2d", self:gethours()) end,
    -- The  hour as a number using a 12-hour clock (01 - 12)
    ['%I']=function(self) return fmt("%.2d", self:getclockhour()) end,
    -- The day of the year as a number (001 - 366)
    ['%j']=function(self) return fmt("%.3d", self:getyearday())  end,
    -- Month of the year, from 01 to 12
    ['%m']=function(self) return fmt("%.2d", self:getmonth())  end,
    -- Minutes after the hour 55
    ['%M']=function(self) return fmt("%.2d", self:getminutes())end,
    -- AM/PM indicator (AM)
    ['%p']=function(self) return sl_meridian[self:gethours() > 11 and 1 or -1] end, --AM/PM indicator (AM)
    -- The second as a number (59, 20 , 01)
    ['%S']=function(self) return fmt("%.2d", self:getseconds())  end,
    -- ISO 8601 day of the week, to 7 for Sunday (7, 1)
    ['%u']=function(self) return self:getisoweekday() end,
    -- Sunday week of the year, from 00 (48)
    ['%U']=function(self) return fmt("%.2d", self:getweeknumber()) end,
    -- ISO 8601 week of the year, from 01 (48)
    ['%V']=function(self) return fmt("%.2d", self:getisoweeknumber()) end,
    -- The day of the week as a decimal, Sunday being 0
    ['%w']=function(self) return self:getweekday() - 1 end,
    -- Monday week of the year, from 00 (48)
    ['%W']=function(self) return fmt("%.2d", self:getweeknumber(2)) end,
    -- The year as a number without a century (range 00 to 99)
    ['%y']=function(self) return fmt("%.2d", mod(self:getyear() ,100)) end,
    -- Year with century (2000, 1914, 0325, 0001)
    ['%Y']=function(self) return fmt("%.4d", self:getyear()) end,
    -- Time zone offset, the date object is assumed local time (+1000, -0230)
    ['%z']=function(self) local b = -self:getbias(); local x = abs(b); return fmt("%s%.4d", b < 0 and "-" or "+", fix(x/60)*100 + floor(mod(x,60))) end,
    -- Time zone name, the date object is assumed local time
    ['%Z']=function(self) return self:gettzname() end,
    -- Misc --
    -- Year, if year is in BCE, prints the BCE Year representation, otherwise result is similar to "%Y" (1 BCE, 40 BCE)
    ['%\b']=function(self) local x = self:getyear() return fmt("%.4d%s", x>0 and x or (-x+1), x>0 and "" or " BCE") end,
    -- Seconds including fraction (59.998, 01.123)
    ['%\f']=function(self) local x = self:getfracsec() return fmt("%s%.9f",x >= 10 and "" or "0", x) end,
    -- percent character %
    ['%%']=function(self) return "%" end,
    -- Group Spec --
    -- 12-hour time, from 01:00:00 AM (06:55:15 AM); same as "%I:%M:%S %p"
    ['%r']=function(self) return self:fmt0("%I:%M:%S %p") end,
    -- hour:minute, from 01:00 (06:55); same as "%I:%M"
    ['%R']=function(self) return self:fmt0("%I:%M")  end,
    -- 24-hour time, from 00:00:00 (06:55:15); same as "%H:%M:%S"
    ['%T']=function(self) return self:fmt0("%H:%M:%S") end,
    -- month/day/year from 01/01/00 (12/02/79); same as "%m/%d/%y"
    ['%D']=function(self) return self:fmt0("%m/%d/%y") end,
    -- year-month-day (1979-12-02); same as "%Y-%m-%d"
    ['%F']=function(self) return self:fmt0("%Y-%m-%d") end,
    -- The preferred date and time representation;  same as "%x %X"
    ['%c']=function(self) return self:fmt0("%x %X") end,
    -- The preferred date representation, same as "%a %b %d %\b"
    ['%x']=function(self) return self:fmt0("%a %b %d %\b") end,
    -- The preferred time representation, same as "%H:%M:%\f"
    ['%X']=function(self) return self:fmt0("%H:%M:%\f") end,
    -- GroupSpec --
    -- Iso format, same as "%Y-%m-%dT%T"
    ['${iso}'] = function(self) return self:fmt0("%Y-%m-%dT%T") end,
    -- http format, same as "%a, %d %b %Y %T GMT"
    ['${http}'] = function(self) return self:fmt0("%a, %d %b %Y %T GMT") end,
    -- ctime format, same as "%a %b %d %T GMT %Y"
    ['${ctime}'] = function(self) return self:fmt0("%a %b %d %T GMT %Y") end,
    -- RFC850 format, same as "%A, %d-%b-%y %T GMT"
    ['${rfc850}'] = function(self) return self:fmt0("%A, %d-%b-%y %T GMT") end,
    -- RFC1123 format, same as "%a, %d %b %Y %T GMT"
    ['${rfc1123}'] = function(self) return self:fmt0("%a, %d %b %Y %T GMT") end,
    -- asctime format, same as "%a %b %d %T %Y"
    ['${asctime}'] = function(self) return self:fmt0("%a %b %d %T %Y") end,
  }
  function dobj:fmt0(str) return (gsub(str, "%%[%a%%\b\f]", function(x) local f = tvspec[x];return (f and f(self)) or x end)) end
  function dobj:fmt(str)
    str = str or self.fmtstr or fmtstr
    return self:fmt0((gmatch(str, "${%w+}")) and (gsub(str, "${%w+}", function(x)local f=tvspec[x];return (f and f(self)) or x end)) or str)
  end

  function dobj.__lt(a, b) if (a.daynum == b.daynum) then return (a.dayfrc < b.dayfrc) else return (a.daynum < b.daynum) end end
  function dobj.__le(a, b) if (a.daynum == b.daynum) then return (a.dayfrc <= b.dayfrc) else return (a.daynum <= b.daynum) end end
  function dobj.__eq(a, b)return (a.daynum == b.daynum) and (a.dayfrc == b.dayfrc) end
  function dobj.__sub(a,b)
    local d1, d2 = date_getdobj(a), date_getdobj(b)
    local d0 = d1 and d2 and date_new(d1.daynum - d2.daynum, d1.dayfrc - d2.dayfrc)
    return d0 and d0:normalize()
  end
  function dobj.__add(a,b)
    local d1, d2 = date_getdobj(a), date_getdobj(b)
    local d0 = d1 and d2 and date_new(d1.daynum + d2.daynum, d1.dayfrc + d2.dayfrc)
    return d0 and d0:normalize()
  end
  function dobj.__concat(a, b) return tostring(a) .. tostring(b) end
  function dobj:__tostring() return self:fmt() end

  function dobj:copy() return date_new(self.daynum, self.dayfrc) end

--[[ THE LOCAL DATE OBJECT METHODS ]]--
  function dobj:tolocal()
    local dn,df = self.daynum, self.dayfrc
    local bias  = getbiasutc2(self)
    if bias then
      -- utc = local + bias; local = utc - bias
      self.daynum = dn
      self.dayfrc = df - bias*TICKSPERSEC
      return self:normalize()
    else
      return nil
    end
  end

  function dobj:toutc()
    local dn,df = self.daynum, self.dayfrc
    local bias  = getbiasloc2(dn, df)
    if bias then
      -- utc = local + bias;
      self.daynum = dn
      self.dayfrc = df + bias*TICKSPERSEC
      return self:normalize()
    else
      return nil
    end
  end

  function dobj:getbias()  return (getbiasloc2(self.daynum, self.dayfrc))/SECPERMIN end

  function dobj:gettzname()
    local _, tvu, _ = getbiasloc2(self.daynum, self.dayfrc)
    return tvu and osdate("%Z",tvu) or ""
  end

--#if not DATE_OBJECT_AFX then
  function date.time(h, r, s, t)
    h, r, s, t = tonumber(h or 0), tonumber(r or 0), tonumber(s or 0), tonumber(t or 0)
    if h and r and s and t then
       return date_new(DAYNUM_DEF, makedayfrc(h, r, s, t))
    else
      return date_error_arg()
    end
  end

  function date:__call(arg1, ...)
    local arg_count = select("#", ...) + (arg1 == nil and 0 or 1)
    if arg_count  > 1 then return (date_from(arg1, ...))
    elseif arg_count == 0 then return (date_getdobj(false))
    else local o, r = date_getdobj(arg1);  return r and o:copy() or o end
  end

  date.diff = dobj.__sub

  function date.isleapyear(v)
    local y = fix(v);
    if not y then
      y = date_getdobj(v)
      y = y and y:getyear()
    end
    return isleapyear(y+0)
  end

  function date.epoch() return date_epoch:copy()  end

  function date.isodate(y,w,d) return date_new(makedaynum_isoywd(y + 0, w and (w+0) or 1, d and (d+0) or 1), 0)  end
  function date.setcenturyflip(y)
    if y ~= floor(y) or y < 0 or y > 100 then date_error_arg() end
    centuryflip = y
  end
  function date.getcenturyflip() return centuryflip end

-- Internal functions
  function date.fmt(str) if str then fmtstr = str end; return fmtstr end
  function date.daynummin(n)  DAYNUM_MIN = (n and n < DAYNUM_MAX) and n or DAYNUM_MIN  return n and DAYNUM_MIN or date_new(DAYNUM_MIN, 0):normalize()end
  function date.daynummax(n)  DAYNUM_MAX = (n and n > DAYNUM_MIN) and n or DAYNUM_MAX return n and DAYNUM_MAX or date_new(DAYNUM_MAX, 0):normalize()end
  function date.ticks(t) if t then setticks(t) end return TICKSPERSEC  end
--#end -- not DATE_OBJECT_AFX

  local tm = osdate("!*t", 0);
  if tm then
    date_epoch = date_new(makedaynum(tm.year, tm.month - 1, tm.day), makedayfrc(tm.hour, tm.min, tm.sec, 0))
    -- the distance from our epoch to os epoch in daynum
    DATE_EPOCH = date_epoch and date_epoch:spandays()
  else -- error will be raise only if called!
    date_epoch = setmetatable({},{__index = function() error("failed to get the epoch date") end})
  end

--#if not DATE_OBJECT_AFX then
return date
--#else
--$return date_from
--#end

