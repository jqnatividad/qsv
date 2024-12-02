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

### Fetch with debug trace enabled to see problem with extra whitespace in URL column

Note: if URL is quoted, then there cannot be extra whitespace before quotes

```
$ cat test4.csv 
City,URL
Beverley Hills, http://geodb-free-service.wirefreethought.com/v1/geo/locations/+34.0901-118.4065/nearbyCities
San Francisco, "http://geodb-free-service.wirefreethought.com/v1/geo/locations/+37.7864-122.3892/nearbyCities"
Anaheim," http://geodb-free-service.wirefreethought.com/v1/geo/locations/+33.8085-117.9228/nearbyCities"

$ QSV_LOG_LEVEL=debug qsv fetch URL test4.csv  --store-error --jql '"data".[0]."name","data".[1]."name","data".[2]."name"' 
[00:00:01] [==================== 100% of 3 records. Cache hit ratio: 0.00% - 3 entries] (7/sec)
"Universal City, Hollywood, Sherman Oaks"
builder error: relative URL without a base
"Anaheim, Garden Grove, Fullerton"

$ grep ERROR qsv_rCURRENT.log | tail -1
[2022-01-06 20:55:49.814944 +08:00] ERROR [qsv::cmd::fetch] src/cmd/fetch.rs:238: Cannot fetch url: "\"http://geodb-free-service.wirefreethought.com/v1/geo/locations/+37.7864-122.3892/nearbyCities\"", error: reqwest::Error { kind: Builder, source: RelativeUrlWithoutBase }

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

### Fetch using custom headers for api key

```
$ cat test5.csv
URL
http://httpbin.org/get

$ qsv fetch URL test5.csv --jql '"headers"."X-Api-Key","headers"."X-Api-Secret"' --store-error --http-header "X-Api-Key:mykey" --http-header "X-Api-Secret  : nottelling"
[00:00:00] [==================== 100% of 1 records. Cache hit ratio: 0.00% - 1 entries] (1,151/sec)
"mykey, nottelling"

$ qsv fetch URL test5.csv --store-error --http-header "X-Api-Key:mykey" --http-header "X-Api-Secret  : nottelling"
[00:00:00] [==================== 100% of 1 records. Cache hit ratio: 0.00% - 1 entries] (1,105/sec)
"{
  ""args"": {}, 
  ""headers"": {
    ""Accept"": ""*/*"", 
    ""Host"": ""httpbin.org"", 
    ""User-Agent"": ""qsv/0.28.0 (https://github.com/dathere/qsv)"", 
    ""X-Amzn-Trace-Id"": ""Root=1-61d8d957-054da2374e304c7c7395cacc"", 
    ""X-Api-Key"": ""mykey"", 
    ""X-Api-Secret"": ""nottelling""
  }, 
  ""origin"": ""1.163.34.120"", 
  ""url"": ""http://httpbin.org/get""
}
"
```
