let locations = require("./locations.json")
const parse = require('csv-parse/lib/sync')
const geolib = require("geolib");
const fs = require("fs");

const airports = parse(fs.readFileSync("./airports.csv", "utf8"), {columns: true, skip_empty_lines:true})

const mapped = {};

locations.results.forEach(location => {
        location.pads.forEach(pad => {
            let closest = {
                distance: 1000*10000,
                location: null,
            }

            airports.forEach(airport => {
                let dist = geolib.getPreciseDistance(
                    {latitude: pad.latitude, longitude: pad.longitude},
                    {latitude: airport.lat, longitude: airport.lon}
                )

                if ( dist < closest.distance ) {
                    closest = {
                        distance: dist,
                        location: airport
                    }
                }

            })

            pad.weather_info = closest;

            if(mapped[pad.id]===undefined) mapped[pad.id] = {}
            mapped[pad.id] = pad
            console.log(`("${pad.latitude}", "${pad.longitude}") => (${pad.weather_info.distance}, ${pad.id}, "${pad.weather_info.location.icao}", "${pad.weather_info.location.name}", "${pad.weather_info.location.country}"),`)
        })
})

fs.writeFileSync("./pad_map.json",JSON.stringify(mapped));