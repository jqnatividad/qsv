# Fetch command

## jql ##

Fetch is integrated with [`jql`](https://github.com/yamafaktory/jql), with some limitations:

* Primary usecase is to retrieve simple values from API JSON response, hence selectors must result in: Number, String, Bool, or Array of such.
* Arrays of Number, String, and Bool are concatenated into String with comma separator
* Output may contain jql error messages
* JSON Null becomes "null" string
* Processing aborts if jql output is still a JSON


## Usage Examples

__test.csv__

```
Country,ZipCode,URL
US,90210,http://api.zippopotam.us/us/90210
US,94105,http://api.zippopotam.us/us/94105
US,92802,http://api.zippopotam.us/us/92802
```


### Fetch url from 3rd column of `test.csv`, apply JQL selector, and print results

```
$ qsv fetch  3 --jql '."places"[0]."place name"' test.csv
Beverly Hills
San Francisco
Anaheim
```

### Fetch url from `URL` column of `test.csv`, apply JQL selector, and put results into new column named City

```
$ qsv fetch URL --new-column City --jql '."places"[0]."place name"' test.csv
Country,ZipCode,URL,City
US,90210,http://api.zippopotam.us/us/90210,Beverly Hills
US,94105,http://api.zippopotam.us/us/94105,San Francisco
US,92802,http://api.zippopotam.us/us/92802,Anaheim
```

### Fetch url from `URL` column of `test.csv`, use JQL to select multiple values, and put them into new column

Please note, multiple values get concatenated into a single quoted string with comma as separator.

```
$ qsv fetch URL --new-column CityState --jql '"places"[0]."place name","places"[0]."state abbreviation"' test.csv
[00:00:00] [==================== 100% of 3 records. Cache hit ratio: 0.00% - 3 entries] (7/sec)
Country,ZipCode,URL,CityState
US,90210,http://api.zippopotam.us/us/90210,"Beverly Hills, CA"
US,94105,http://api.zippopotam.us/us/94105,"San Francisco, CA"
US,92802,http://api.zippopotam.us/us/92802,"Anaheim, CA"
```

### Fetch with explicit rate limit set, and pipe output to a new csv

```
$ qsv fetch URL test.csv --rate-limit 3 --jql '"places"[0]."longitude","places"[0]."latitude"' -c Coordinates > new.csv
[00:00:01] [==================== 100% of 3 records. Cache hit ratio: 0.00% - 3 entries] (5/sec)
$ cat new.csv
Country,ZipCode,URL,Coordinates
US,90210,http://api.zippopotam.us/us/90210,"-118.4065, 34.0901"
US,94105,http://api.zippopotam.us/us/94105,"-122.3892, 37.7864"
US,92802,http://api.zippopotam.us/us/92802,"-117.9228, 33.8085"
```

