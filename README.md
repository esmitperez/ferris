# Ferris
Ferris is a Rust-based Terminal interface to display data on Puget Sound WSDOT Ferries.

Potential features:
- Real time location tracking.
- Ferry catalog (a la [sample ferry info page](https://wsdot.com/ferries/vesselwatch/VesselDetail.aspx?vessel_id=36)).
- Route planner ("should I drive, or take the ferry?").

# Why?

Been meaning to use my (lack of) Rust skills into something practical that combines my personal life passions: Maps, Transportation infrastructure and Continuous Learning.

# How?

I'll base the mapping logic on TUI's own [Canvas example](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/canvas.rs), the in-line comments refer to [Source data](http://www.gnuplotting.org/plotting-the-world-revisited), which describes how the data could've been originally extracted for that project.


My approach will be:
- Use OpenStreetMap data to extract: the Puget Sound coastline, ferry routes and city names
- Convert it to a vector, similar to how TUI did it
- Provide a demo app that only displays the above, in much the same way [WSDOTs Real Time Map](https://wsdot.com/ferries/vesselwatch/default.aspx?ula=47.614213&ulo=-122.483255&z=11)  does
- Figure out how to retrieve VesselWatch data
- Figure out how to display such data in the map.

### Puget Sound Coastline Extraction

At this time, OSM restricts how much data you can download, they suggest using Planet OSM, but I found a service that offers Seattle area OSM extracts, https://www.interline.io/osm/extracts/.

Other good options seem to be:

- [bbbike.org](https://download.bbbike.org/osm/extract)
- WSDOT's own [Geospatial Open Data Portal](https://gisdata-wsdot.opendata.arcgis.com/)
- [WSDOT - Ferry Routes](https://wsdot.maps.arcgis.com/home/item.html?id=fad5f3f332f74b3388070e9c96a0ba35)
- [WSDOT - Ferry Terminals, Public and Private](https://wsdot.maps.arcgis.com/home/item.html?id=094582d874004735a2732d967b24d5aa)


- Downloaded https://download.bbbike.org/osm/extract/planet_-122.812,47.474_-122.235,47.761.osm.shp.zip
- Added as Vector Layer in QGIS
- Saved "roads" layer as `pugetsound.geojson`
- Ran this
```shell
cat pugetsound.geojson| jq -c '.features[] | select (.type == "Feature" and .properties.type == "motorway") | .geometry.coordinates[][] |  [ (.[0]*1000|round/1000) , (.[1]*1000|round/1000) ] ' |  sed  's/\]/),/gi' | tr '[' '(' | pbcopy
```

- Appended `| pbcopy` to be able to copy paste into `PUGET_HIGH_RESOLUTION` in file `world.rs` and replaced that with `wc -l` in order to count how many entries to especify in same file.

### Ferry Route Data Set 

Sources:.
- [GeoJSON](https://data.wsdot.wa.gov/arcgis/rest/services/Shared/FerryRoutes/MapServer/1/query?outFields=*&where=1%3D1&f=geojson) in [WSDOT - Ferry Routes](https://geo.wa.gov/datasets/WSDOT::wsdot-ferry-routes/explore?location=47.552965%2C-120.820450%2C8.00)'s Open API Explorer

# When?

As my personal time allows.


# Who?

My name is Esmit Perez, long time Software Developer-turned Engineering Manager. 

Here's where to find me:
Website [https://esmit.me/](https://esmit.me/)
LinkedIn [/in/esmit](linkedin.com/in/esmit)
Twitter [@mitiwifi](twitter.com/@mitiwifi)


# References

The following tutorials were used as "inspiration" for the basic skeleton:

- [Rust and TUI: Building a command-line interface in Rust](https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/)