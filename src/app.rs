extern crate reqwest;
use std::error::Error;

use geojson::{GeoJson, Geometry, Value};
// use reqwest::{ClientBuilder, error::Error};


pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub terminals: Vec<Terminal<'a>>,
    pub routes: Vec<Route<'a>>,
    pub enhanced_graphics: bool,
}

pub struct Terminal<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub coords: (f64, f64),
    pub status: &'a str,
}

#[derive(Debug, Clone)]
pub enum RouteType {
    State,
    County
}

#[derive(Debug, Clone)]
pub struct Route<'a> {
    pub name: &'a str,
    pub route_type: RouteType
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        
        let app = App {
            title,
            should_quit: false,
            terminals: vec![
                Terminal {
                    name: "Seattle",
                    location: "Seattle",
                    coords: (47.60, -122.33),
                    status: "Up",
                },
                Terminal {
                    name: "Bremerton",
                    location: "Bremerton",
                    coords: (47.56, -122.62),
                    status: "F",
                },
                // Route {
                //     name: "Costa Rica",
                //     location: "SJO",
                //     coords: (9.74, -83.75),
                //     status: "Failure",
                // },
            ],
            routes: vec![],
            enhanced_graphics,
        };

        app
    }

    pub fn on_up(&mut self) {
        // self.tasks.previous();
    }

    pub fn on_down(&mut self) {
        // self.tasks.next();
    }

    pub fn on_right(&mut self) {
        // self.tabs.next();
    }

    pub fn on_left(&mut self) {
        // self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
    pub fn on_tick(&mut self) {}

    pub async fn load_ferry_routes(&self) -> reqwest::Result<()> {
    println!("load_ferry_routes");

        let resp = reqwest::Client::builder()
    .danger_accept_invalid_certs(true)
    .build()
    .unwrap()
    .get("https://data.wsdot.wa.gov/arcgis/rest/services/Shared/FerryRoutes/MapServer/1/query?outFields=*&where=1=1&f=geojson")
    .send()
        .await?
        .json::<GeoJson>()
        .await?;

        // pretty print the whole GeoJSON response
        // println!("{resp:#?}");

        match resp {
            GeoJson::FeatureCollection(ref ctn) => {
                println!("FeatureCollection");
                for feature in &ctn.features {
                    if let Some(ref geom) = feature.geometry {
                        // for prop in &feature.properties {
                        //     for v in prop {
                        //         println!("{} = {}", v.0, v.1)
                        //     }
                        // }

                        self.match_geometry(geom)
                    }
                }

                let filtered = &ctn
                    .features
                    .iter()
                    .filter_map(|feature| {
                        for prop in &feature.properties {
                            let mut marked = false;
                            let mut name = "";
                            for v in prop {
                                
                                // is this a state or county route?
                                if "State" == v.1 || "County" == v.1 {
                                    marked = true;
                                } 

                                // extract the route name
                                if "Display" == v.0 {
                                    name = v.1.as_str().unwrap();
                                } else {
                                    println!("Field name {}", v.0)
                                }
                                
                            }
                            if marked {
                                return Some(name);
                            }
                        }
                        None
                    })
                    .filter_map(|f| {
                        if f.is_empty() {
                            return None
                        }
                        println!("{f:?}");
                        Some(Route{
                            name: f,
                            route_type: RouteType::State
                        })
                    // }).for_each(|f| {
                    //     println!("{f:#?}");
                    }).reduce(|f| {

                    });

                let x:Vec<Route> = filtered.red
                // println!("{x:#?}");
                //self.update_routes(c);
            }
            GeoJson::Feature(ref feature) => {
                println!("Feature");
                if let Some(ref geom) = feature.geometry {
                    self.match_geometry(geom)
                }
            }
            GeoJson::Geometry(ref geometry) => self.match_geometry(geometry),
        }
        // println!("{:#?}", resp);
        Ok(())
    }

    fn update_routes<'b> (&self, routes: Vec<Route>){
        // self.routes = routes;
    }

    /// Process GeoJSON geometries
    fn match_geometry(&self, geom: &Geometry) {
        match &geom.value {
            Value::LineString(l) => {
                println!("Matched a LineString");
                l.iter().for_each(|v| {
                    println!("{},{}", v[0], v[1])
                })
            },
            Value::Polygon(_) => println!("Matched a Polygon"),
            Value::MultiPolygon(_) => println!("Matched a MultiPolygon"),
            Value::GeometryCollection(ref gc) => {
                println!("Matched a GeometryCollection");
                // !!! GeometryCollections contain other Geometry types, and can
                // nest — we deal with this by recursively processing each geometry
                for geometry in gc {
                    self.match_geometry(geometry)
                }
            }
            // Point, LineString, and their Multi– counterparts
            _ => println!("Matched some other geometry"),
        }
    }
}
