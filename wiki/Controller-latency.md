Controller tracking will always be difficult. There are so many factors that influence the latency and the motion prediction. Its not something like "100ms" constantly, but depends on your movements and even the movement of the headset.

There are many parameters that influence the movement that can be changed:

- Tracking is currently async to the rendering and running at 3*72=216Hz
- Movement prediction is set to 0 to get the latest tracking info -> no prediction on the quest
- Tracking info is sent to SteamVR
- Tracking info is fed into SteamVR with an offset of 10ms to enable SteamVR pose prediction
- The tracking point on the Quest is different that the point on the Rift S. Angular acceleration and linear acceleration of the controller needed to be transformed to the new reference.

There is a trade off between fast but wobbly and overshooting controllers and controllers that have a certain latency.

You can change settings related to prediction in "Settings -> Headset -> Controllers"