### A whirlwind tour

Let's say you're playing with some of the data from the
[Data Science Toolkit](https://github.com/petewarden/dstkdata), which contains
several CSV files. Maybe you're interested in the population counts of each
city in the world. So grab the 124MB, 2.7M row CSV file and start examining it:

```bash
$ curl -LO https://raw.githubusercontent.com/petewarden/dstkdata/master/worldcitiespop.csv
# there are no headers in the file, so let's get the headers
$ curl -LO https://raw.githubusercontent.com/jqnatividad/qsv/master/resources/whirlwind_tour/worldcitiespop-header.csv
# and preppend the header. On Linux and macOS do
$ cat worldcitiespop-header.csv worldcitiespop.csv > wcp.csv
# on Windows Powershell
$ Get-Content worldcitiespop-header.csv, worldcitiespop.csv | Out-File wcp.csv -Encoding utf8
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

```bash
$ qsv stats wcp.csv --everything | qsv table
field       type     sum                min           max         min_length  max_length  mean                stddev              variance            lower_fence         q1          q2_median   q3          iqr                upper_fence         skew                  mode         cardinality  nullcount
Country     String                      ad            zw          2           2                                                                                                                                                                                            ru           231          0
City        String                       al lusayli   Þykkvibaer  1           87                                                                                                                                                                                           san jose     2008182      0
AccentCity  String                       Al Lusayli   özlüce      1           87                                                                                                                                                                                           San Antonio  2031214      0
Region      String                      00            Z4          0           2                                                                       -29.5               5           11          28          23                 62.5                1.3036035769599401    04           392          4
Population  Integer  2290536125         7             31480498    0           8           48730.66387966977   308414.0418510231   95119221210.88461   -33018              3730.5      10879       28229.5     24499              64978               0.36819008290764255                28460        2652350
Latitude    Float    76585211.19776328  -54.9333333   82.483333   1           12          28.371681223643343  21.938373536960917  481.292233447227    -35.9076389         12.9552778  33.8666667  45.5305556  32.5752778         94.3934723          -0.7514210842155992   50.8         255133       0
Longitude   Float    75976506.66429423  -179.9833333  180         1           14          28.14618114715278   62.472858625866486  3902.8580648875004  -98.49166745000002  2.383333    26.8802778  69.6333333  67.25000030000001  170.50833375000002  0.060789759344963286  23.1         407568       0
```

The `qsv table` command takes any CSV data and formats it into aligned columns
using [elastic tabstops](https://github.com/BurntSushi/tabwriter). You'll
notice that it even gets alignment right with respect to Unicode characters.

So, this command took 4.73 seconds to run on my machine, but we can speed
it up by creating an index and re-running the command:

```bash
$ qsv index wcp.csv
$ qsv stats wcp.csv --everything | qsv table
```

Which cuts it down to 1.99 seconds on my machine. (And creating the index
took 0.26 seconds.)

Notably, the same type of "statistics" command in another
[CSV command line toolkit](https://csvkit.readthedocs.io/)
takes about 8 minutes to produce similar statistics on the same data set.

Creating an index gives us more than just faster statistics gathering. It also
makes slice operations extremely fast because *only the sliced portion* has to
be parsed. For example, let's say you wanted to grab the last 10 records:

```bash
$ qsv count wcp.csv
2699354
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

These commands are *instantaneous* because they run in time and memory
proportional to the size of the slice (which means they will scale to
arbitrarily large CSV data).

Hmmmm... the Population column has a lot of null values. How pervasive is that?
First, let's take a look at 10 "random" rows with `sample`. We use the `--seed` parameter
so we get a reproducible random sample. And then, let's display only the Country,
AccentCity and Population columns with the `select` command.

```bash
$ qsv sample --seed 42 10 wcp.csv \
  | qsv select Country,AccentCity,Population \
  | qsv table
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

Whoops! The sample we got don't have population counts. How pervasive is that?

```bash
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

```bash
$ qsv search --select Population '[0-9]' wcp.csv \
  | qsv sample --seed 42 10 \
  | qsv select Country,AccentCity,Population \
  | tee sample.csv \
  | qsv table
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

> :warning: **NOTE:** The `tee` command reads from standard input and writes 
to both standard output and one or more files at the same time. We do this so 
we can create the `sample.csv` file we need for the next step, and pipe the 
same data to the `qsv table` command.

Erk. Which country is `sv`? What continent? No clue, but [datawookie](https://github.com/datawookie) 
has a CSV file called `country-continent.csv`.

```bash
$ curl -L https://raw.githubusercontent.com/datawookie/data-diaspora/master/spatial/country-continent-codes.csv > country_continent.csv
$ qsv headers country_continent.csv
1 # https://datahub.io/JohnSnowLabs/country-and-continent-codes-list
```

Huh!?! That's not what we we were expecting. But if you look at the `country-continent.csv` file, it starts with a comment starting
with the `#` character. No worries, qsv got us covered with its `QSV_COMMENT_CHAR` environment variable.

```bash
$ export QSV_COMMENT_CHAR='#'
# on Windows Powershell
$ $env:QSV_COMMENT_CHAR='#'
$ qsv headers country_continent.csv
1   continent
2   code
3   country
4   iso2
5   iso3
6   number
```
That's more like it. We can now do a join to see which countries and continents these are:

```bash
$ qsv join --no-case Country sample.csv iso2 country_continent.csv  | qsv table
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

```bash
$ qsv join --no-case Country sample.csv iso2 country_continent.csv \
  | qsv select 'AccentCity,Population,country,continent' \
  | qsv dedup --select 'country,AccentCity' \
  | qsv rename City,Population,Country,Continent \
  | qsv table
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

Perhaps we can do this with the original CSV data? All 2.7 million rows in a 124MB file?!  

Indeed we can—because `qsv` is designed for speed - written in [Rust](https://www.rust-lang.org/) with 
[amortized memory allocations](https://blog.burntsushi.net/csv/#amortizing-allocations), using the 
performance-focused [mimalloc](https://github.com/microsoft/mimalloc) allocator.

```bash
$ qsv join --no-case Country wcp.csv iso2 country_continent.csv \
  | qsv search --select Population '[0-9]' \
  | qsv select 'AccentCity,Population,country,continent,Latitude,Longitude' \
  | qsv dedup --select 'country,AccentCity,Latitude,Longitude' --dupes-output wcp_dupes.csv \
  | qsv rename City,Population,Country,Continent,Latitude,Longitude --output wcp_countrycontinent.csv

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

$ qsv count wcp_countrycontinent.csv
47004
$ qsv count wcp-dupes.csv
5155
```

We fine-tuned `dedup` by adding `Latitude` and `Longitude` as there may be 
multiple cities with the same name in a country. We also specified the 
`dupes-output` option so we can have a separate CSV of the duplicate records
it removed.

We're also just interested in cities with population counts. So we used `search`
with the regular expression `[0-9]`. This cuts down the file to 47,004 rows.

This whole thing takes about 5 seconds on my machine. The performance of `join`,
in particular, comes from constructing a very simple hash index of one of the CSV 
files. The `join` command does an inner join by default, but it also has left,
right and full outer, cross, anti and semi join support too. All from the command line,
without having to load the files into a database, index them, to do a SQL join.

Finally, can we create a CSV file for each country of all its cities? Yes we can, with
the `partition` command (and it took just 0.04 seconds to create all 211 country-city files!):

```bash
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

The US population is more than 179,123,400 (Population sum) and 3,439 cities (City cardinality).
Perhaps we can get population info elsewhere with the `fetch` command...
But that's another tour by itself! 😄
