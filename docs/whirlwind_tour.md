### A whirlwind tour

> ℹ️ **NOTE:** This tour is primarily targeted to Linux and macOS users. Though qsv works on Windows, the tour
assumes basic knowledge of command-line piping and redirection, and uses other command-line tools (curl, tee, head, etc.)
that are not installed by default on Windows.

Let's say you're playing with some data from the
[Data Science Toolkit](https://github.com/petewarden/dstkdata), which contains
several CSV files. Maybe you're interested in the population counts of each
city in the world. So grab the 124MB, 2.7M row CSV file and start examining it:

```
# there are no headers in the original repo, so let's download a prepared CSV with headers
$ curl -LO https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/wcp.zip
$ unzip wcp.zip
$ qsv headers wcp.csv
1   Country
2   City
3   AccentCity
4   Region
5   Population
6   Latitude
7   Longitude
```

The next thing you might want to do is get an overview of the kind of data that
appears in each column. The `stats` command will do this for you:

```
$ qsv stats wcp.csv | qsv table
field       type     sum                min           max          min_length  max_length  mean                stddev              variance           nullcount
Country     String                      ad            zw           2           2                                                                      0
City        String                       al lusayli   ??ykkvibaer  1           87                                                                     0
AccentCity  String                       Al Lusayli   ??zl??ce     1           87                                                                     0
Region      String                      00            Z4           0           2                                                                      4
Population  Integer  2290536128         3             31480498     0           8           48729.62723114559   308410.84307353816  95117248125.33058  2652349
Latitude    Float    76585211.1977638   -54.9333333   82.483333    1           12          28.371681223642454  21.938373536961045  481.2922334472327  0
Longitude   Float    75976506.66428813  -179.9833333  180.0        1           14          28.14618114715136   62.472858625866586  3902.858064887513  0
```

Wow! That was fast! It took just 1.3 seconds to compile all that.[^1] One reason for qsv's speed
is that ***it mainly works in "streaming" mode*** - computing statistics as it "streams"
the CSV file line by line. This also means it can gather statistics on arbitrarily large files,
as it does not have to load the entire file into memory.[^2]

But can we get more summary statistics? What's the variance, the modes, the distribution (quartiles), 
and the cardinality of the data?  No problem. That's why `qsv stats` has an `--everything` option to 
compute these more "expensive" stats. Expensive - as these extended statistics can only be computed at 
the cost of loading the entire file into memory.

```
$ qsv stats wcp.csv --everything | qsv table
field       type     sum                min           max          min_length  max_length  mean                stddev              variance           nullcount  lower_outer_fence    lower_inner_fence   q1          q2_median   q3          iqr                upper_inner_fence   upper_outer_fence   skewness              mode         cardinality
Country     String                      ad            zw           2           2                                                                      0                                                                                                                                                                        ru           231
City        String                       al lusayli   ??ykkvibaer  1           87                                                                     0                                                                                                                                                                        san jose     2008182
AccentCity  String                       Al Lusayli   ??zl??ce     1           87                                                                     0                                                                                                                                                                        San Antonio  2031214
Region      String                      00            Z4           0           2                                                                      4                                                                                                                                                                        04           392
Population  Integer  2290536128         3             31480498     0           8           48729.627231145605  308410.84307353816  95117248125.33058  2652349    -69768.5             -33019.25           3730.0      10879.0     28229.5     24499.5            64978.75            101728.0            0.4163962529847548                 28461
Latitude    Float    76585211.1977638   -54.9333333   82.483333    1           12          28.37168122364246   21.938373536961045  481.2922334472327  0          -84.7705556          -35.9076389         12.9552778  33.8666667  45.5305556  32.5752778         94.3934723          143.256389          -0.28388092518431285  50.8         255133
Longitude   Float    75976506.66428813  -179.9833333  180.0        1           14          28.146181147151353  62.472858625866586  3902.858064887513  0          -199.36666790000004  -98.49166745000002  2.383333    26.8802778  69.6333333  67.25000030000001  170.50833375000002  271.38333420000004  0.2714663289005219    23.1         407568

```

> ℹ️ **NOTE:** The `qsv table` command takes any CSV data and formats it into aligned columns
using [elastic tabstops](https://github.com/BurntSushi/tabwriter). You'll
notice that it even gets alignment right with respect to Unicode characters.

So, this command took 3.22 seconds to run on my machine, but we can speed
it up by creating an index and re-running the command:

```
qsv index wcp.csv
qsv stats wcp.csv --everything | qsv table
```

Which cuts it down to 1.95 seconds - 1.65x faster! (And creating the 21.6mb index took 0.27 seconds. 
What about the first `stats` without `--everything`? From 1.3 seconds to 0.16 seconds with an index - 8.25x faster!)

Notably, the same type of "statistics" command in another
[CSV command line toolkit](https://csvkit.readthedocs.io/)
takes about 10 seconds to produce a *subset* of statistics on the same data set. [Visidata](https://visidata.org)
takes much longer - ~1.5 minutes to calculate a *subset* of these statistics with its Describe sheet. 
Even python [pandas'](https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.describe.html) 
`describe(include="all"))` took 12 seconds to calculate a *subset* of qsv's "streaming" statistics.[^3]

This is another reason for qsv's speed. Creating an index accelerated statistics gathering as it enables 
***multithreading & fast I/O***.

**For multithreading** - running `stats` with an index was 8.25x faster because it divided the file into 
16 equal chunks[^1] with ~170k records each, then running stats on each chunk in parallel across 16 
logical processors and merging the results in the end. It was "only" 8x, and not 16x faster as there is 
some overhead involved in multithreading. 

**For fast I/O** - let's say you wanted to grab the last 10 records:

```
$ qsv count --human-readable wcp.csv
2,699,354
$ qsv slice wcp.csv --start -10 | qsv table
Country  City               AccentCity         Region  Population  Latitude     Longitude
zw       zibalonkwe         Zibalonkwe         06                  -19.8333333  27.4666667
zw       zibunkululu        Zibunkululu        06                  -19.6666667  27.6166667
zw       ziga               Ziga               06                  -19.2166667  27.4833333
zw       zikamanas village  Zikamanas Village  00                  -18.2166667  27.95
zw       zimbabwe           Zimbabwe           07                  -20.2666667  30.9166667
zw       zimre park         Zimre Park         04                  -17.8661111  31.2136111
zw       ziyakamanas        Ziyakamanas        00                  -18.2166667  27.95
zw       zizalisari         Zizalisari         04                  -17.7588889  31.0105556
zw       zuzumba            Zuzumba            06                  -20.0333333  27.9333333
zw       zvishavane         Zvishavane         07      79876       -20.3333333  30.0333333
```

`qsv count` took 0.006 seconds and `qsv slice`, 0.017 seconds! These commands are *instantaneous* 
with an index because for `count` - the index already precomputed the record count, and with `slice`,
*only the sliced portion* has to be parsed - because an index allowed us to jump directly to that 
part of the file. It didn't have to scan the entire file to get the last 10 records. For comparison,
without an index, it took 0.25 (41x slower) and 0.66 (39x slower) seconds respectively.

> ℹ️ **NOTE:** Creating/updating an index itself is extremely fast as well. If you want
qsv to automatically create and update indices, set the environment var `QSV_AUTOINDEX`.

Okay, okay! Let's switch gears and stop obsessing over how fast :rocket: qsv is... let's go back to exploring :mag_right:
the data set.

Hmmmm... the Population column has a lot of null values. How pervasive is that?
First, let's take a look at 10 "random" rows with `sample`. We use the `--seed` parameter
so we get a reproducible random sample. And then, let's display only the Country,
AccentCity and Population columns with the `select` command.

```
$ qsv sample --seed 42 10 wcp.csv | 
    qsv select Country,AccentCity,Population | 
    qsv table
Country  AccentCity            Population
ar       Colonia Santa Teresa  
ro       Piscu Scoartei        
gr       Liáskovo              
de       Buntenbeck            
tr       Mehmetçelebi Köyü     
pl       Trzeciewiec           
ar       Colonias Unidas       
at       Koglhof               
bg       Nadezhda              
ru       Rabog                 
```

Whoops! The sample we got doesn't have population counts. It's quite pervasive.
Exactly how many cities have empty (NULL) population counts?

```
$ qsv frequency wcp.csv --limit 3 | qsv table
field       value        count
Country     ru           176934
Country     us           141989
Country     cn           117508
City        san jose     313
City        san antonio  310
City        santa rosa   288
AccentCity  San Antonio  307
AccentCity  Santa Rosa   288
AccentCity  Santa Cruz   268
Region      04           143900
Region      02           127736
Region      03           105455
Population  (NULL)       2652350
Population  2310         12
Population  2137         11
Latitude    50.8         1128
Latitude    50.95        1076
Latitude    50.6         1043
Longitude   23.1         590
Longitude   23.2         586
Longitude   23.05        575
```

(The `qsv frequency` command builds a frequency table for each column in the
CSV data. This one only took 1.8 seconds.)

So it seems that most cities do not have a population count associated with
them at all (2,652,350 to be exact). No matter — we can adjust our previous 
command so that it only shows rows with a population count:

```
$ qsv search --select Population '[0-9]' wcp.csv |
    qsv sample --seed 42 10 |
    qsv select Country,AccentCity,Population |
    tee sample.csv |
    qsv table
Country  AccentCity         Population
it       Isernia            21409
lt       Ramygala           1637
ro       Band               7599
in       Nagapattinam       94247
hn       El Negrito         9304
us       North Druid Hills  21320
gb       Ellesmere Port     67768
bd       Parbatipur         48026
sv       Apastepeque        5785
ge       Lajanurhesi        95
```

> ℹ️ **NOTE:** The `tee` command reads from standard input and writes 
to both standard output and one or more files at the same time. We do this so 
we can create the `sample.csv` file we need for the next step, and pipe the 
same data to the `qsv table` command.<br/>Why create `sample.csv`? Even though qsv is blazing-fast, we're just doing an 
initial investigation and a small 10-row sample is all we need to try out and
compose the different CLI commands needed to wrangle the data.

Erk. Which country is `sv`? What continent? No clue, but [datawookie](https://github.com/datawookie) 
has a CSV file called `country-continent.csv`.

```
$ curl -L https://raw.githubusercontent.com/datawookie/data-diaspora/master/spatial/country-continent-codes.csv > country_continent.csv
$ qsv headers country_continent.csv
1 # https://datahub.io/JohnSnowLabs/country-and-continent-codes-list
```

Huh!?! That's not what we were expecting. But if you look at the `country-continent.csv`
file, it starts with a comment with the `#` character. 

```
$ head -5 country_continent.csv
# https://datahub.io/JohnSnowLabs/country-and-continent-codes-list
continent,code,country,iso2,iso3,number
Asia,AS,"Afghanistan, Islamic Republic of",AF,AFG,4
Europe,EU,"Albania, Republic of",AL,ALB,8
Antarctica,AN,Antarctica (the territory South of 60 deg S),AQ,ATA,10
```

No worries, qsv got us covered with its `QSV_COMMENT_CHAR` environment variable. Setting it
to `#` tells qsv to ignore any lines in the CSV - may it be before the header, or even in the data
part of the CSV, that **starts with the character** we set it to.

```
$ export QSV_COMMENT_CHAR='#'

$ qsv headers country_continent.csv
1   continent
2   code
3   country
4   iso2
5   iso3
6   number
```

That's more like it. We can now do a join to see which countries and continents these are:

```
$ qsv join --ignore-case Country sample.csv iso2 country_continent.csv  | qsv table
Country  AccentCity         Population  continent      code  country                                             iso2  iso3  number
it       Isernia            21409       Europe         EU    Italy, Italian Republic                             IT    ITA   380
lt       Ramygala           1637        Europe         EU    Lithuania, Republic of                              LT    LTU   440
ro       Band               7599        Europe         EU    Romania                                             RO    ROU   642
in       Nagapattinam       94247       Asia           AS    India, Republic of                                  IN    IND   356
hn       El Negrito         9304        North America  NA    Honduras, Republic of                               HN    HND   340
us       North Druid Hills  21320       North America  NA    United States of America                            US    USA   840
gb       Ellesmere Port     67768       Europe         EU    United Kingdom of Great Britain & Northern Ireland  GB    GBR   826
bd       Parbatipur         48026       Asia           AS    Bangladesh, People's Republic of                    BD    BGD   50
sv       Apastepeque        5785        North America  NA    El Salvador, Republic of                            SV    SLV   222
ge       Lajanurhesi        95          Europe         EU    Georgia                                             GE    GEO   268
ge       Lajanurhesi        95          Asia           AS    Georgia                                             GE    GEO   268

```

`sv` is El Salvador - never would have guessed that. Thing is, now we have several unneeded
columns, and the column names case formats are not consistent. Also, there are two records
for Lajanurhesi - for both Europe and Asia. This is because Georgia spans both continents.  
We're primarily interested in unique cities per country for the purposes of this tour,
so we need to filter these out.

Also, apart from renaming the columns, I want to reorder them to "City, Population, Country,
Continent".

No worries. Let's use the `select` (so we only get the columns we need, in the order we want), 
`dedup` (so we only get unique County/City combinations) and `rename` (columns in titlecase) commands: 

```
$ qsv join --ignore-case Country sample.csv iso2 country_continent.csv |
    qsv select 'AccentCity,Population,country,continent' |
    qsv dedup --select 'country,AccentCity' |
    qsv rename City,Population,Country,Continent |
    qsv table
City               Population  Country                                             Continent
Parbatipur         48026       Bangladesh, People's Republic of                    Asia
Apastepeque        5785        El Salvador, Republic of                            North America
Lajanurhesi        95          Georgia                                             Asia
El Negrito         9304        Honduras, Republic of                               North America
Nagapattinam       94247       India, Republic of                                  Asia
Isernia            21409       Italy, Italian Republic                             Europe
Ramygala           1637        Lithuania, Republic of                              Europe
Band               7599        Romania                                             Europe
Ellesmere Port     67768       United Kingdom of Great Britain & Northern Ireland  Europe
North Druid Hills  21320       United States of America                            North America
```

Nice! Notice the data is now sorted by Country,City too! That's because `dedup` first sorts the
CSV records (by internally calling the `qsv sort` command) to find duplicates.  

Now that we've composed all the commands we need, perhaps we can do this with the original CSV data? 
Not the tiny 10-row sample.csv file, but all 2.7 million rows in the 124MB `wcp.csv` file?!  

Indeed we can — because `qsv` is designed for speed - written in [Rust](https://www.rust-lang.org/) with 
[amortized memory allocations](https://blog.burntsushi.net/csv/#amortizing-allocations), using the 
performance-focused [mimalloc](https://github.com/microsoft/mimalloc) allocator.

```
$ qsv join --ignore-case Country wcp.csv iso2 country_continent.csv |
    qsv search --select Population '[0-9]' |
    qsv select 'AccentCity,Population,country,continent,Latitude,Longitude' |
    qsv dedup --select 'country,AccentCity,Latitude,Longitude' --dupes-output wcp_dupes.csv |
    qsv rename City,Population,Country,Continent,Latitude,Longitude --output wcp_countrycontinent.csv

$ qsv sample 10 --seed 33 wcp_countrycontinent.csv | qsv table
City            Population  Country                       Continent      Latitude    Longitude
Santa Catalina  2727        Philippines, Republic of the  Asia           16.0822222  120.6097222
Azacualpa       1258        Honduras, Republic of         North America  14.7166667  -88.1
Solana          2984        Philippines, Republic of the  Asia           8.6230556   124.7705556
Sungai Besar    26939       Malaysia                      Asia           3.6666667   100.9833333
Bad Nenndorf    10323       Germany, Federal Republic of  Europe         52.3333333  9.3666667
Dalwangan       4906        Philippines, Republic of the  Asia           8.2030556   125.0416667
Sharonville     13250       United States of America      North America  39.2680556  -84.4133333
El Calvario     557         Colombia, Republic of         South America  4.3547222   -73.7091667
Kunoy           70          Faroe Islands                 Europe         62.2833333  -6.6666667
Lufkin          33667       United States of America      North America  31.3380556  -94.7288889

$ qsv count -H wcp_countrycontinent.csv
47,004
$ qsv count -H wcp-dupes.,csv
5,155
```

We fine-tuned `dedup` by adding `Latitude` and `Longitude` as there may be 
multiple cities with the same name in a country. We also specified the 
`dupes-output` option so we can have a separate CSV of the duplicate records
it removed.

We're also just interested in cities with population counts. So we used `search`
with the regular expression `[0-9]`. This cuts down the file to 47,004 rows.

**The whole thing took ~5 seconds on my machine.** The performance of `join`,
in particular, comes from constructing a [SIMD](https://www.sciencedirect.com/topics/computer-science/single-instruction-multiple-data)-accelerated hash index of one of the CSV 
files. The `join` command does an inner join by default, but it also has left,
right and full outer, cross, anti and semi join support too. All from the command line,
without having to load the files into a database, index them, to do a SQL join.

Finally, can we create a CSV file for each country of all its cities? Yes we can, with
the `partition` command (and it took just 0.04 seconds to create all 211 country-city files!):

```
$ qsv partition Country bycountry wcp_countrycontinent.csv
$ cd bycountry
$ ls -1shS
total 164M
320K UnitedStatesofAmerica.csv
264K PhilippinesRepublicofthe.csv
256K RussianFederation.csv
172K IndiaRepublicof.csv
...
4.0K DjiboutiRepublicof.csv
4.0K Aruba.csv
4.0K Anguilla.csv
4.0K Gibraltar.csv
4.0K Ukraine.csv
```

Examining the USA csv file:

```
$ qsv stats --everything UnitedStatesofAmerica.csv | qsv table --output usa-cities-stats.csv
$ less -S usa-cities-stats.csv
field       type     sum                 min                       max                       min_length  max_length  mean                stddev              variance           lower_fence         q1           q2_median    q3           iqr                upper_fence          skew                 mode                                                          cardinality  nullcount
City        String                       Abbeville                 Zionsville                3           26                                                                                                                                                                                             Springfield                                                   3439         0
Population  Integer  179123400           216                       8107916                   3           7           42903.80838323359   167752.88891786628  28141031740.28998  -24217.5            12081        19235        36280        24199              72578.5              0.4232798946578281   10576,10945,11971,12115,13219,13250,8771,9944                 3981         0
Country     String                       United States of America  United States of America  24          24                                                                                                                                                                                             United States of America                                      1            0
Continent   String                       North America             North America             13          13                                                                                                                                                                                             North America                                                 1            0
Latitude    Float    158455.7901657997   17.9677778                71.2905556                10          10          37.95348267444306   6.0032154906925355  36.03859622769082  22.244444449999992  34.0552778   39.4694444   41.9291667   7.873888900000004  53.740000050000006   -0.7575748669562047  42.0333333                                                    4010         0
Longitude   Float    -377616.7797696997  -165.4063889              -65.3013889               11          12          -90.44713287897018  17.2089567990395    296.1481941112077  -128.2138889        -97.4863889  -86.0341667  -77.0013889  20.485             -46.273888899999996  -0.769302793394743   -118.3516667,-71.0666667,-71.3972222,-71.4166667,-83.1500000  4074         0
```

Hhhmmm... clearly the worldcitiespop.csv file from the Data Science Toolkit does not have 
comprehensive coverage of City populations.

The US population is far more than 179,123,400 (Population sum) and 3,439 cities (City cardinality).
Perhaps we can get population info elsewhere with the `fetch` command...
But that's another tour by itself! 😄

[^1]: Timings collected by setting `QSV_LOG_LEVEL='debug'` on a Ryzen 4800H laptop (8 physical/16 logical cores) running Windows 11 with 32gb of memory and a 1 TB SSD.
[^2]: For example, running `qsv stats` on a CSV export of ALL of NYC's available 311 data from 2010 to Mar 2022 (27.8M rows, 16gb) took just 22.4 seconds with an index (which actually took longer to create - 39 seconds to create a 223mb index), and its memory footprint remained the same, pinning all 16 logical processors near 100% utilization on my Ryzen 7 4800H laptop with 32gb memory and 1 TB SSD.
[^3]: [Why is qsv exponentially faster than python pandas?](https://github.com/dathere/datapusher-plus/discussions/15)
