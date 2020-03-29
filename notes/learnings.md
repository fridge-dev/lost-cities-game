These are my learnings which I will apply on the next ascii-board game I make.

== General ==

* UI and backend should have different GameApi APIs and type definitions
  * UI sometimes has subset of data or formatted differently

== UI ==

* State is difficult, can be more easily managed if everything is strictly modeled as a screen
* "Smart" CLI should be inside each screen mod
* Everything should run from a screen, and screens should redirect to other screens
* Having my own draw trait might be easier than `impl Display`

== Backend ==

* "Dumb" storage layer was good

