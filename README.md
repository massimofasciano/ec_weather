## ec_weather

Get current weather conditions from Environment Canada at this site:
https://dd.weather.gc.ca/citypage_weather/xml/

Data is parsed from XML and displayed either in plaintext (temperature or relative humidity) or
as a JSON object containing the current conditions.

Intermediate Rust data structures are used to deserialize from XML and then serialize to JSON.
Most elements in the data structure use the Option type so it should deal with missing fields in the XML.
The XML date is converted into an rfc3339 timestamp (ex: 2023-04-16T03:00:00Z).

This version has a proper command line interface and processes the data during processing.
A simpler version without this processing is available here:
https://github.com/massimofasciano/ec_weather_simple

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
          The ID of the weather station see https://dd.weather.gc.ca/citypage_weather/xml/siteList.xml

          [default: s0000635]

  -l, --language <LANGUAGE>
          The language of text info in the XML data (currently only affects text summary of date)

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
   "dewpoint" : {
      "unitType" : "metric",
      "value" : 1.2,
      "units" : "C"
   },
   "wind" : {
      "direction" : "E",
      "gust" : {
         "units" : "km/h",
         "unitType" : "metric"
      },
      "bearing" : {
         "value" : 95.7,
         "units" : "degrees"
      },
      "speed" : {
         "units" : "km/h",
         "value" : 2,
         "unitType" : "metric"
      }
   },
   "relativeHumidity" : {
      "value" : 86,
      "units" : "%"
   },
   "pressure" : {
      "value" : 101.1,
      "units" : "kPa",
      "unitType" : "metric"
   },
   "windChill" : {
      "unitType" : "metric",
      "value" : -1
   },
   "visibility" : {
      "units" : "km",
      "unitType" : "metric"
   },
   "dateTime" : [
      {
         "minute" : 0,
         "year" : 2023,
         "UTCOffset" : 0,
         "textSummary" : "16 avril 2023 03h00 UTC",
         "hour" : 3,
         "zone" : "UTC",
         "month" : 4,
         "day" : 16
      },
      {
         "month" : 4,
         "day" : 15,
         "minute" : 0,
         "year" : 2023,
         "textSummary" : "15 avril 2023 22h00 HNE",
         "UTCOffset" : -5,
         "hour" : 22,
         "zone" : "HNE"
      }
   ],
   "temperature" : {
      "unitType" : "metric",
      "units" : "C",
      "value" : 3.4
   }
}

$ ec_weather -p bc -s s0000873
{
   "error" : "Failed to fetch weather data for https://dd.weather.gc.ca/citypage_weather/xml/BC/s0000873_e.xml with status: 404 Not Found"
}
```