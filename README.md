# NextLaunch
Watch a countdown until the next rocket launch, live in your terminal!!


## Usage
Simply launch the program from your terminal, or file explorer, and it will work out of the box.

To view credits, launch the program via the terminal with the `-c` or `--credits` flags


## Installation
To see how to install NextLaunch, please check the [latest release](https://github.com/fatalcenturion/NextLaunch/releases/latest)

## Previews
### Default View

Without any flight telemetry available for the upcoming launch, the default view will look similar to this most of the time:
```
+======================================================================= NextLaunch ======================================================================+
| Mission:              Starlink 17 | Status:                     Go For Launch | State:                      Clear | Humidity:                       52% |
| Vehicle:         Falcon 9 Block 5 | Window Open:  Feb 2nd 2021 - 10:14:00 GMT | Wind Direction:  North North East | Air Pressure:           1024.5 MBar |
| Location:      Launch Complex 39A | Window Close: Feb 2nd 2021 - 10:14:00 GMT | Wind Speed:                 3 MPH | Temperature:                   12 C |
| Provider:                  SpaceX | T-0:          Feb 2nd 2021 - 10:14:00 GMT | Wind Bearing:          27 Degrees | Minimum Temperature:            7 C |
| Destination:      Low Earth Orbit | Likelihood:                           70% | Visibility:              14 Miles | Maximum Temperatrue:           14 C |
+=========== Mission Info ==========+======+======== Status Info ===============+========================== Weather Information ==========================+
| Total Launch Count:                  122 |                                                                                                              |
| Successful Launches:                 113 |                                                                                                              |
| Failed Launches:                       8 |                                                                                                              |
| Pending Launches:                     35 |                                                                                                              |
| Consecutive Successes:                 1 |                                                                                                              |
+============ Launch Statistics ===========+                                                                                                              |
| Attempted Landings:                   96 |                                                                                                              |
| Successful Landings:                  81 |                                                                                                              |
| Failed Landings:                      14 |                                                                                                              |
| Consecutive Landings:                  1 |                                                                                                              |
+=========== Landing Statistics ===========+                                                                                                              |
| Launch Type:                  Commercial |                                                                                                              |
| Provider CEO:                  Elon Musk |                                                                                                              |
| Founded:                            2002 |                                                                                                              |
+========= Additional Information =========+                                                                                                              |
| Status:                           Online |                                                                                                              |
| Webcast:                          Online |                                                                                                              |
| Telemetry:                        Online |                                                                                                              |
| Last Update:    1 minute, 32 seconds ago |                                                                                                              |
| NextLaunch V1.0.0 (Phoenix Rising)       |                                                                                                              |
+=========================================+======================+====== Countdown ==+====================+===============================================+
|                                                                                                                                                         |
|                                 #####          #####  #####        ###    #####        #  #   #####        #####  #####                                 |
|                                   #            #   #  #   #   ##     #        #   ##   #  #   #   #   ##   #          #                                 |
|                                   #    #####   #   #  #   #          #      ###        #####  #   #        #####  #####                                 |
|                                   #            #   #  #   #   ##     #        #   ##      #   #   #   ##       #  #                                     |
|                                   #            #####  #####        #####  #####           #   #####        #####  #####                                 |
|                                                                                                                                                         |
+=========================================================================================================================================================+
```

The default view will automatically switch into a "connected" state when the countdown gets within a certain distance from `T0`, this is what that "connected" state could look like:

```
+======================================================================= NextLaunch ======================================================================+
| Mission:              Starlink 17 | Status:                     Go For Launch | State:                      Clear | Humidity:                       52% |
| Vehicle:         Falcon 9 Block 5 | Window Open:  Feb 2nd 2021 - 10:14:00 GMT | Wind Direction:  North North East | Air Pressure:           1024.5 mBar |
| Location:      Launch Complex 39A | Window Close: Feb 2nd 2021 - 10:14:00 GMT | Wind Speed:                 3 MPH | Temperature:                   12 C |
| Provider:                  SpaceX | T-0:          Feb 2nd 2021 - 10:14:00 GMT | Wind Bearing:          27 Degrees | Minimum Temperature:            7 C |
| Destination:      Low Earth Orbit | Likelihood:                           70% | Visibility:              14 Miles | Maximum Temperatrue:           14 C |
+=========== Mission Info ==========+======+======== Status Info ===============+========================== Weather Information ==========================+
| Total Launch Count:                  122 | View on:                                                                                                     |
| Successful Launches:                 113 | Flight Club:      https://flightclub.io/result/2d?id=272f4f21-0998-4078-9094-d1867dcb897d                    |
| Failed Launches:                       8 | Space Launch Now: https://spacelaunchnow.me/launch/falcon-9-block-5-starlink-17                              |
| Pending Launches:                     35 | Go 4 Liftoff:     https://go4liftoff.com/launch/falcon-9-block-5-starlink-17                                 |
| Consecutive Successes:                 1 | Nextlaunch Web:   https://nextlaunch.org/rooms/falcon-9-block-5-starlink-17                                  |
+============ Launch Statistics ===========+                                                                                                              |
| Attempted Landings:                   96 |                                                                                                              |
| Successful Landings:                  81 |                                                                                                              |
| Failed Landings:                      14 |                                                                                                              |
| Consecutive Landings:                  1 |                                                                                                              |
+=========== Landing Statistics ===========+                                                                                                              |
| Launch Type:                  Commercial |                                                                                                              |
| Provider CEO:                  Elon Musk |                                                                                                              |
| Founded:                            2002 | Velocity: [XXXXXXXXXXXXXXXXXXXXXXX                                                           ]     7165 km/h |
+========= Additional Information =========+======================================+============================ Flight Events ============================+
| Status:                           Online |                                      | Time                                                             Name |
| Webcast:                          Online | Distance Travelled:          5.47 Km | T+ 156 seconds                                     Main Engine Cutoff |
| Telemetry:                        Online | Acceleration:                18.5 Gs | T+ 91 seconds                                             Throttle Up |
| Last Update:    1 minute, 32 seconds ago | Angle:                    52 Degrees | T+ 74 seconds                                                   Max Q |
| NextLaunch V1.0.0 (Phoenix Rising)       | Aerodynamic Pressure: 30,479 Pascals | T+ 53 seconds                                           Throttle Down |
+=========================================+======================+====== Countdown ==+====================+===============================================+
|                                                                                                                                                         |
|                                 #####          #####  #####        ###    #####        #  #   #####        #####  #####                                 |
|                                   #      #     #   #  #   #   ##     #        #   ##   #  #   #   #   ##   #          #                                 |
|                                   #    #####   #   #  #   #          #      ###        #####  #   #        #####  #####                                 |
|                                   #      #     #   #  #   #   ##     #        #   ##      #   #   #   ##       #  #                                     |
|                                   #            #####  #####        #####  #####           #   #####        #####  #####                                 |
|                                                                                                                                                         |
+=========================================================================================================================================================+
```

## Roadmap

Upcoming features i wish to add to the program.
- Telemetry mode: a near-real time telemetry view of the current launch, featuring things like vertical and horizontal speed, as well as altitude.

- Launch specification: A way of simply following the progress of a single launch at a time, instead of the program deciding what is best.

- Low power mode: A mode which will throttle the refresh time from 1 second, to five seconds, in the default view. Allowing for an even more efficient version of the program.

- custom user color schemes for different launch status types

- Historical launch "replays" for the telemtry mode: does what it says on the tin, allows you to specify a launch, for the telemetry mode to replay in a 1 second refresh view

- "Wiki" mode: A way of looking at information on a specific launch, agency, rocket, launchpad, or country (generated as the data is located, to prevent API overload)

<div align="center">
    <h5 align="center">This program is provided free of charge, with no warranty, or guarantee of future updates.</h5>
</div>
// TODO: `.nd`-like launch forecast tool

