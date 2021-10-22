### A whirlwind tour

Let's say you're playing with some of the data from the
[Data Science Toolkit](https://github.com/petewarden/dstkdata), which contains
several CSV files. Maybe you're interested in the population counts of each
city in the world. So grab the data and start examining it:

```bash
$ curl -LO https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/worldcitiespop_mil.zip
$ unzip worldcitiespop_mil.zip
$ qsv headers worldcitiespop.csv
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
$ qsv stats worldcitiespop.csv --everything | qsv table
field       type     sum                 min            max            min_length  max_length  mean                stddev              variance            median      mode         cardinality  nullcount
Country     Unicode                      ad             zw             2           2                                                                                   cn           234          0
City        Unicode                       bab el ahmar  Þykkvibaer     1           91                                                                                  san jose     2351892      0
AccentCity  Unicode                       Bâb el Ahmar  ïn Bou Chella  1           91                                                                                  San Antonio  2375760      0
Region      Unicode                      00             Z9             0           2                                                                       13          04           397          8
Population  Integer  2289584999          7              31480498       0           8           47719.570633597126  302885.5592040396   91739661974.34377   10779                    28754        3125978
Latitude    Float    86294096.37312101   -54.933333     82.483333      1           12          27.188165808468785  21.95261384912504   481.91725480879654  32.4972221  51.15        1038349      0
Longitude   Float    117718483.57958724  -179.9833333   180            1           14          37.08885989656418   63.223010459241635  3997.1490515293776  35.28       23.8         1167162      0
```

The `qsv table` command takes any CSV data and formats it into aligned columns
using [elastic tabstops](https://github.com/BurntSushi/tabwriter). You'll
notice that it even gets alignment right with respect to Unicode characters.

So, this command takes about 12 seconds to run on my machine, but we can speed
it up by creating an index and re-running the command:

```bash
$ qsv index worldcitiespop.csv
$ qsv stats worldcitiespop.csv --everything | qsv table
...
```

Which cuts it down to about 8 seconds on my machine. (And creating the index
takes less than 2 seconds.)

Notably, the same type of "statistics" command in another
[CSV command line toolkit](https://csvkit.readthedocs.io/)
takes about 2 minutes to produce similar statistics on the same data set.

Creating an index gives us more than just faster statistics gathering. It also
makes slice operations extremely fast because *only the sliced portion* has to
be parsed. For example, let's say you wanted to grab the last 10 records:

```bash
$ qsv count worldcitiespop.csv
3173958
$ qsv slice worldcitiespop.csv -s 3173948 | qsv table
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

Switching gears a little bit, you might not always want to see every column in
the CSV data. In this case, maybe we only care about the country, city and
population. So let's take a look at 10 "random" rows. We use the `--seed` parameter
so we get a reproducible random sample:

```bash
$ qsv select Country,AccentCity,Population worldcitiespop.csv \
  | qsv sample --seed 42 10 \
  | qsv table
Country  AccentCity       Population
vn       Khánh Tàn     
no       Kvalvåg       
ir       Bala Dashteh  
af       Kam Papin     
cn       Peipiao       
mz       Chiquefane    
ug       Bulukatoni    
us       Gourdsville   
kr       Hwahungni     
fr       Pimouget
```

Whoops! The sample we got don't have population counts. How pervasive is that?

```bash
$ qsv frequency worldcitiespop.csv --limit 5
field,value,count
Country,cn,238985
Country,ru,215938
Country,id,176546
Country,us,141989
Country,ir,123872
City,san jose,328
City,san antonio,320
City,santa rosa,296
City,santa cruz,282
City,san juan,255
AccentCity,San Antonio,317
AccentCity,Santa Rosa,296
AccentCity,Santa Cruz,281
AccentCity,San Juan,254
AccentCity,San Miguel,254
Region,04,159916
Region,02,142158
Region,07,126867
Region,03,122161
Region,05,118441
Population,(NULL),3125978
Population,2310,12
Population,3097,11
Population,983,11
Population,2684,11
Latitude,51.15,777
Latitude,51.083333,772
Latitude,50.933333,769
Latitude,51.116667,769
Latitude,51.133333,767
Longitude,23.8,484
Longitude,23.2,477
Longitude,23.05,476
Longitude,25.3,474
Longitude,23.1,459
```

(The `qsv frequency` command builds a frequency table for each column in the
CSV data. This one only took 5 seconds.)

So it seems that most cities do not have a population count associated with
them at all (3,125,978 to be exact). No matter — we can adjust our previous 
command so that it only shows rows with a population count:

```bash
$ qsv search -s Population '[0-9]' worldcitiespop.csv \
  | qsv select Country,AccentCity,Population \
  | qsv sample --seed 42 10 \
  | tee sample.csv \
  | qsv table
Country  AccentCity       Population
fr       Boissy-Saint-Léger  15451
us       Iselin              17019
ru       Ali-Yurt            7593
ro       Panaci              2308
lu       Baschleiden         185
us       Mayaguez            76503
ch       Vernier             29767
es       Salobreña           10725
ch       Aigle               7897
yt       Ouangani            7273
```

> :warning: **NOTE:** The `tee` command reads from standard input and writes 
to both standard output and one or more files at the same time. We do this so 
we can create the `sample.csv` file we need for the next step, and pipe the 
same data to the `qsv table` command.

Erk. Which country is `yt`? What continent? No clue, but [DataHub.io](https://datahub.io) 
has a CSV file called `country-continent.csv`. Let's grab it and do a join so 
we can see which countries and continents these are:

```bash
curl -L https://datahub.io/JohnSnowLabs/country-and-continent-codes-list/r/0.csv > country_continent.csv
$ qsv headers countrynames.csv
1   Continent_Name
2   Continent_Code
3   Country_Name
4   Two_Letter_Country_Code
5   Three_Letter_Country_Code
6   Country_Number
$ qsv join --no-case Country sample.csv Two_Letter_Country_Code country_continent.csv  | qsv table
Country  AccentCity          Population  Continent_Name  Continent_Code  Country_Name                      Two_Letter_Country_Code  Three_Letter_Country_Code  Country_Number
fr       Boissy-Saint-Léger  15451       Europe          EU              France, French Republic           FR                       FRA                        250
us       Iselin              17019       North America   NA              United States of America          US                       USA                        840
ru       Ali-Yurt            7593        Europe          EU              Russian Federation                RU                       RUS                        643
ru       Ali-Yurt            7593        Asia            AS              Russian Federation                RU                       RUS                        643
ro       Panaci              2308        Europe          EU              Romania                           RO                       ROU                        642
lu       Baschleiden         185         Europe          EU              Luxembourg, Grand Duchy of        LU                       LUX                        442
us       Mayaguez            76503       North America   NA              United States of America          US                       USA                        840
ch       Vernier             29767       Europe          EU              Switzerland, Swiss Confederation  CH                       CHE                        756
es       Salobreña           10725       Europe          EU              Spain, Kingdom of                 ES                       ESP                        724
ch       Aigle               7897        Europe          EU              Switzerland, Swiss Confederation  CH                       CHE                        756
yt       Ouangani            7273        Africa          AF              Mayotte                           YT                       MYT                        175

```

Whoops, now we have the data but we have several unneeded columns, and the columns
we do need have overly long names. Also, there are two records for Ali-Yurt - one in
Europe and another in Asia. This is because Russia spans both continents.  
We're primarily interested in unique cities per country for the purposes of this tour,
so we need to filter these out.

Also, apart from renaming the columns, I want to reorder them to "Country,Continent,City,
Population".

No worries. Let's use the `select` (so we only get the columns we need, in the order we want), 
`dedup` (so we only get unique County/City combinations) and `rename` (rename the columns 
with shorter names) commands: 

```bash
$ qsv join --no-case Country sample.csv Two_Letter_Country_Code country_continent.csv \
  | qsv select 'Country_Name,Continent_Name,AccentCity,Population' \
  | qsv dedup -s 'Country_Name,AccentCity' \
  | qsv rename Country,Continent,City,Population \
  | qsv table
Country                           Continent      City                Population
France, French Republic           Europe         Boissy-Saint-Léger  15451
Luxembourg, Grand Duchy of        Europe         Baschleiden         185
Mayotte                           Africa         Ouangani            7273
Romania                           Europe         Panaci              2308
Russian Federation                Asia           Ali-Yurt            7593
Spain, Kingdom of                 Europe         Salobreña           10725
Switzerland, Swiss Confederation  Europe         Aigle               7897
Switzerland, Swiss Confederation  Europe         Vernier             29767
United States of America          North America  Iselin              17019
United States of America          North America  Mayaguez            76503
```

Nice! Notice the data is now sorted too! That's because `dedup` first sorts the
CSV records (by internally calling the `qsv sort` command) to find duplicates.  

Perhaps we can do this with the original CSV data? All 3.2 million rows in a 145MB file?!  

Indeed we can—because `qsv` is designed for speed - written in [Rust](https://www.rust-lang.org/) with 
[amortized memory allocations](https://blog.burntsushi.net/csv/#amortizing-allocations), using the 
performance-focused [mimalloc](https://github.com/microsoft/mimalloc) allocator.

```bash
$ qsv join --no-case Country worldcitiespop.csv Two_Letter_Country_Code country_continent.csv \
  | qsv select 'Country_Name,Continent_Name,AccentCity,Population,Latitude,Longitude' \
  | qsv dedup -s 'Country_Name,AccentCity,Latitude,Longitude' --dupes-output dupe-countrycities.csv \
  | qsv rename Country,Continent,City,Population,Latitude,Longitude \
  > worldcitiespop_countrycontinent.csv
$ qsv sample 10 --seed 1729 worldcitiespop_countycontinent.csv | qsv table
Country                            Continent      City                     Population  Latitude    Longitude
Syrian Arab Republic               Asia           Cheikh Sayad                         36.25       37.2666667
Colombia, Republic of              South America  Tetillo                              3.160288    -76.324643
Egypt, Arab Republic of            Africa         El-Kôm el-Ahmar                      27.0        31.4166667
Bulgaria, Republic of              Europe         Oresha                               42.95       24.1
Poland, Republic of                Europe         Wielka Wies                          54.568121   17.361634
Iran, Islamic Republic of          Asia           Kolah Jub-e Chagalvandi              33.65       48.583333
Congo, Republic of the             Africa         Ebou                                 -1.2833333  15.5869444
Congo, Democratic Republic of the  Africa         Yambo-Engunda                        1           20.666667
Australia, Commonwealth of         Oceania        Cessnock                 16394       -32.832111  151.356232
Brazil, Federative Republic of     South America  Pirapora                             -8.448056   -72.821389
```

We fine-tuned `dedup` by adding `Latitude` and `Longitude` as there may be 
multiple cities with the same name in a country. We also specified the 
`dupes-output` option so we can have a separate CSV of the duplicate records
it removed. 

This whole thing takes about 8 seconds on my machine. The performance of `join`,
in particular, comes from constructing a very simple hash index of one of the CSV 
files. The `join` command does an inner join by default, but it also has left,
right and full outer join support too.

Finally, can we create a CSV file for each country of all its cities? Yes we can, 
with the `partition` command:

```bash
$ qsv partition Country bycountry worldcitiespop_countrycontinent.csv
$ cd bycountry
$ ls -1shS
total 191M
 16M ChinaPeoplesRepublicof.csv
 12M RussianFederation.csv
 11M UnitedStatesofAmerica.csv
 11M IndonesiaRepublicof.csv
7.5M IranIslamicRepublicof.csv
...
4.0K CocosKeelingIslands.csv
4.0K PitcairnIslands.csv
4.0K Tokelau.csv
4.0K NorfolkIsland.csv
```
