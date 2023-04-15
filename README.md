## ec_weather

Get current weather conditions from Environment Canada from this site:
https://dd.weather.gc.ca/citypage_weather/xml/

Data is parsed from XML and displayed either in plaintext (temperature or relative humidity) or as a
JSON object containing the current conditions.

Most elements in the data structure use the Option type so it should deal with missing fields in the XML.
In the JSON output they appear as null.

```
$ ec_weather --help

Get current weather conditions from Environment Canada

Usage: ec_weather [OPTIONS]

Options:
  -u, --url <URL>
          The full URL for the XML data (only needed if they change the site layout)

  -t, --temperature-only
          Only display the temperature

  -r, --relative-humidity-only
          Only display the humidity

  -p, --province <PROVINCE>
          In which province is the weather station ?

          [default: qc]
          [possible values: ab, bc, hef, mb, nb, nl, ns, nt, nu, on, pe, qc, sk, yt]

  -s, --station-id <STATION_ID>
          The ID of the weather station ? see https://dd.weather.gc.ca/citypage_weather/xml/siteList.xml

          [default: s0000635]

  -l, --language <LANGUAGE>
          The language of text info in the XML data ? (currently only affects text summary of date)

          [default: english]

          Possible values:
          - french:  XML text info in french
          - english: XML text info in english

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Some examples:

```
$ ec_weather -t
10.8

$ ec_weather -r
48

$ ec_weather -p on -s s0000073 -l french
{
   "pressure" : {
      "value" : 101.3,
      "unitType" : "metric",
      "units" : "kPa"
   },
   "temperature" : {
      "unitType" : "metric",
      "units" : "C",
      "value" : 14.3
   },
   "windChill" : {
      "value" : -1,
      "unitType" : "metric",
      "units" : null
   },
   "humidex" : null,
   "relativeHumidity" : {
      "value" : 50,
      "unitType" : null,
      "units" : "%"
   },
   "dewpoint" : {
      "value" : 4,
      "unitType" : "metric",
      "units" : "C"
   },
   "visibility" : {
      "unitType" : "metric",
      "units" : "km",
      "value" : null
   },
   "dateTime" : [
      {
         "year" : 2023,
         "UTCOffset" : 0,
         "minute" : 0,
         "zone" : "UTC",
         "month" : 4,
         "day" : 15,
         "textSummary" : "15 avril 2023 14h00 UTC",
         "hour" : 14
      },
      {
         "year" : 2023,
         "minute" : 0,
         "UTCOffset" : -5,
         "zone" : "HNE",
         "month" : 4,
         "day" : 15,
         "textSummary" : "15 avril 2023 09h00 HNE",
         "hour" : 9
      }
   ]
}

$ ec_weather -p bc -s s0000873
{
   "error" : "Failed to fetch weather data for https://dd.weather.gc.ca/citypage_weather/xml/BC/s0000873_e.xml with status: 404 Not Found"
}
```