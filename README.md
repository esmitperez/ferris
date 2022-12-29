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

# When?

As my personal time allows.


# Who?

My name is Esmit Perez, long time Software Developer-turned Engineering Manager. 

Here's where to find me:
Website [https://esmit.me/](https://esmit.me/)
LinkedIn [/in/esmit](linkedin.com/in/esmit)
Twitter [@mitiwifi](twitter.com/@mitiwifi)
