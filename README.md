# Mapfilter

CLI tool to run regex and location based filters on nodes from OpenStreetMap.

## Getting data

Download `.osm.pbf` dump, for example from https://download.geofabrik.de/.

## Usage

Run `mapfilter map.osm.pbf <filters>`.

### Examples

Show cities called "London":
```
$ mapfilter map.osm.pbf -n "^London$" -t "place=city"
┏ London (#1)
┃  📍 http://openstreetmap.org/node/107775
┃  🌍 http://google.com/maps/search/51.50732+-0.12765
┃  🏷️ capital: yes  ele: 15  is_capital: country  name: London
┃     note: Centre of London officially at the former location of the Charing Cross, now the Charles I statue, near Trafalgar Square.
┃     place: city  population: 8908081  website: https://www.london.gov.uk/
┗━━━━
┏ London (#2)
┃  📍 http://openstreetmap.org/node/65606
┃  🏷️ boundary: ceremonial  designation: ceremonial_county  int_name: London
┃     name: London
┃     note: This relation is for the 'county' of Greater London, which excludes the City of London
┃     place: city  type: boundary
┗━━━━

Total nodes: 188_243_033 / Filtered to: 2 / Displayed: 2
```

Show at most five towns, cities or villages that contain the same substring of 4+ characters twice:
```
$ mapfilter map.osm.pbf -m 5 -r "place=(city|town|village)" -f "^name\$=(?i)(....).*\1"
┏ Loughborough (#1)
┃  📍 http://openstreetmap.org/node/10021975
┃  🌍 http://google.com/maps/search/52.77239+-1.20780
┃  🏷️ is_in: Leicestershire, United Kingdom  name: Loughborough  place: town
┃     population: 59317
┗━━━━
┏ North Kilworth (#2)
┃  📍 http://openstreetmap.org/node/27150756
┃  🌍 http://google.com/maps/search/52.44594+-1.09543
┃  🏷️ created_by: JOSM  name: North Kilworth  place: village
┗━━━━
┏ Woolage Village (#3)
┃  📍 http://openstreetmap.org/node/29202181
┃  🌍 http://google.com/maps/search/51.20597+1.19934
┃  🏷️ is_in: Kent, England, UK  name: Woolage Village  place: village
┃     source: survey
┗━━━━
┏ Auchtermuchty (#4)
┃  📍 http://openstreetmap.org/node/29622132
┃  🌍 http://google.com/maps/search/56.29208+-3.23283
┃  🏷️ name: Auchtermuchty  place: town  population: 2093  source: npe
┗━━━━
┏ Portree - Port Rìgh (#5)
┃  📍 http://openstreetmap.org/node/46628151
┃  🌍 http://google.com/maps/search/57.41305+-6.19445
┃  🏷️ is_in: Isle of Skye, Highland Region  name: Portree - Port Rìgh
┃     place: town  population: 2318
┗━━━━
┏ Brightwell-cum-Sotwell (#6)
┃  📍 http://openstreetmap.org/node/266672299
┃  🌍 http://google.com/maps/search/51.61551+-1.16521
┃  🏷️ created_by: Potlatch 0.10f  name: Brightwell-cum-Sotwell  place: village
┗━━━━
✂️ Reached output limit, not showing more

Total nodes: 188_243_033 / Filtered to: 27 / Displayed: 5
```

Show places with population above 100k within 25km of a given point:
```
$ mapfilter map.osm.pbf -m 5 -r "population=\d{6}" -l "52.5,-1.5,25000"
┏ Coventry (#1)
┃  📍 http://openstreetmap.org/node/17859918
┃  🌍 http://google.com/maps/search/52.40818+-1.51048
┃  📏 10_234 meters
┃  🏷️ is_in: West Midlands;England;UK  name: Coventry  place: city
┃     population: 337428
┗━━━━
┏ Solihull (#2)
┃  📍 http://openstreetmap.org/node/20980396
┃  🌍 http://google.com/maps/search/52.41302+-1.77689
┃  📏 21_108 meters
┃  🏷️ is_in: West Midlands  name: Solihull  place: town
┃     population: 206091  source:population: council tax bill
┗━━━━
┏ Hinckley (#3)
┃  📍 http://openstreetmap.org/node/26679131
┃  🌍 http://google.com/maps/search/52.54106+-1.37294
┃  📏 9_734 meters
┃  🏷️ name: Hinckley  place: town  population: 105078
┗━━━━

Total nodes: 188_243_033 / Filtered to: 3 / Displayed: 3
```