# Fetch command

## jql ##

Fetch is integrated with [`jql`](https://github.com/yamafaktory/jql), with some limitations:

* Primary usecase is to retrieve simple values from API JSON response, hence selectors must result in: Number, String, Bool, or Array of such.
* Arrays of Number, String, and Bool are concatenated into String with comma separator
* On jql error, output is blank. To see jql error message please enable `--store-error` flag
* JSON Null becomes `"null"` string
* Processing aborts if jql output is still a JSON


## Usage Examples

__test.csv__

```
Country,ZipCode,URL
US,99999,http://api.zippopotam.us/us/99999
US,90210,http://api.zippopotam.us/us/90210
US,94105,http://api.zippopotam.us/us/94105
US,92802,http://api.zippopotam.us/us/92802
```


### Fetch data via 3rd column of `test.csv`, apply JQL selector, and print results

Notice that first row is blank due to api error (bad url).

```
$ qsv fetch 3 test.csv --jql '"places"[0]."place name"'
[00:00:00] [==================== 100% of 4 records. Cache hit ratio: 0.00% - 4 entries] (6/sec)
""
Beverly Hills
San Francisco
Anaheim
```

### Fetch data via `URL` column of `test.csv`, apply JQL selector, and put results into new column named City

Notice that on error, instead of blank value, error message can be stored via `--store-error` flag.

```
$ qsv fetch URL test.csv --jql '"places"[0]."place name"' -c City --store-error
[00:00:00] [==================== 100% of 4 records. Cache hit ratio: 0.00% - 4 entries] (6/sec)
Country,ZipCode,URL,City
US,99999,http://api.zippopotam.us/us/99999,HTTP 404 - Not Found
US,90210,http://api.zippopotam.us/us/90210,Beverly Hills
US,94105,http://api.zippopotam.us/us/94105,San Francisco
US,92802,http://api.zippopotam.us/us/92802,Anaheim


```

### Fetch data via `URL` column of `test.csv`, use JQL to select multiple values, and put them into new column

Please note, multiple values get concatenated into a single quoted string with comma as separator.

```
$ qsv fetch URL --new-column CityState --jql '"places"[0]."place name","places"[0]."state abbreviation"' test.csv
[00:00:00] [==================== 100% of 4 records. Cache hit ratio: 0.00% - 4 entries] (7/sec)
Country,ZipCode,URL,CityState
US,99999,http://api.zippopotam.us/us/99999,
US,90210,http://api.zippopotam.us/us/90210,"Beverly Hills, CA"
US,94105,http://api.zippopotam.us/us/94105,"San Francisco, CA"
US,92802,http://api.zippopotam.us/us/92802,"Anaheim, CA"
```

### Fetch data via `URL` column of `test.csv`, with invalid jql selector

```
$ qsv fetch URL test.csv --jql '"place"[0]."place name"'
[00:00:01] [==================== 100% of 4 records. Cache hit ratio: 0.00% - 4 entries] (4/sec)
""
""
""
""

$ qsv fetch URL test.csv --jql '"place"[0]."place name"' --store-error
[00:00:01] [==================== 100% of 4 records. Cache hit ratio: 0.00% - 4 entries] (6/sec)
HTTP 404 - Not Found
"Node ""place"" not found on the parent element"
"Node ""place"" not found on the parent element"
"Node ""place"" not found on the parent element"

```

### Fetch with explicit rate limit, and pipe output to a new csv

```
$ qsv fetch URL test.csv --rate-limit 3 --jql '"places"[0]."longitude","places"[0]."latitude"' -c Coordinates > new.csv
[00:00:01] [==================== 100% of 4 records. Cache hit ratio: 0.00% - 4 entries] (4/sec)
$ cat new.csv
Country,ZipCode,URL,Coordinates
US,99999,http://api.zippopotam.us/us/99999,
US,90210,http://api.zippopotam.us/us/90210,"-118.4065, 34.0901"
US,94105,http://api.zippopotam.us/us/94105,"-122.3892, 37.7864"
US,92802,http://api.zippopotam.us/us/92802,"-117.9228, 33.8085"
```

