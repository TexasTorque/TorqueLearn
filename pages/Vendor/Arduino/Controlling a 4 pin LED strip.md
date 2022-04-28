# Controlling a 4 pin LED strip

If your LED strip has 4 pins, RGB and a Ground, this is probably the right way.

## What you need?
* Arduino (we used Arduino UNO)
* The strip
* Jumper wires
* A 12v power supply (your strip is probably 12v, also you might need to sacrifice this)
* A breadboard (for your first build)
* 3x N-channel MOSFETS (per control circuit) Try [these](https://www.amazon.com/WeiMeet-RFP30N06LE-N-Channel-Mosfet-Arduino/dp/B07CTF1JVD/)
* Soldering stuff
* Basic knowledge of breadboarding

## Why use these? Why not?
The strips are super cheap, but the electronics are bulky and annoying sometimes. These are dead easy to program with Arduino though. They're not individually addressable because there's effectively no logic on board the strip, everything's just wired in parallel. That means these strips are cuttable, which is cool.

We're using these as a simpe and cheap way to spice up our driver stations in the 2022 season.

## Hardware setup
First, separate out each of the pads on the strip into a separate Arduino compatible male jumper. Then plug in three sockets on the Arduino (they must have the "~" PWM symbol next to them for this to work properly.) Just use the pins, 3, 5, and 6 on the Arduino UNO for now.

Then on the breadboard place three N-channel MOSFETS near the center, facing out away from the center. Give them some space between each other to make wiring easier. Position the breadboard so that the MOSFETS are lined up vertically in front of you with the longer "tab" part facing towards your right hand, and the darker short part facing to your left. Each of the MOSFETS controls one color.

Now connect one jumper each from the Arduino to the "top" pin of each MOSFET. Then, connect the jumpers from the RGB strips to the "middle" of each MOSFET. Make sure to color code your jumpers so you know what color is controlled by what pin. 

Now it's time for power. Cut your 12v power supply so it branches out into two leads, expose some metal on each lead. Connect a jumper from the "bottom" of each MOSFET to one jumper. Then grab your single ground and single power in and touch them to each side of the power supply until the strip shows some sign of activity. Then you know you've got the polarity right.

## Programming!
In the Arduino software first define three pins, make sure to match color to pin number. Keep in mind these are pre-processor directives, not variable declarations
```C++
#define RED 3
#define GREEN 5
#define BLUE 6
```

The default program should look something like 
```C++
void setup() {

}
void loop() {

}
```
First set the mode of each pin with
```C++
void setup() {
    pinMode(RED, OUTPUT);
    pinMode(GREEN, OUTPUT);
    pinMode(BLUE, OUTPUT);
}
```
The the loop function (which runs continuously) you can simply write an 8-bit RGB value (0-255) to each pin to change the color. For example...
```C++
void loop() {
    analogWrite(RED, 0);
    analogWrite(GREEN, 255);
    analogWrite(BLUE, 0);
    delay(5000);
    analogWrite(RED, 255);
    analogWrite(GREEN, 255);
    analogWrite(BLUE, 255);
    delay(5000);
}
```
Try to figure out what this does.

There's a lot of fun math you can do to make cool affects, like a sort of rainbow shift, try to program that :D.

Btw if you need some help with wiring see the diagram below, but keep in mind it doesn't incorporate the need for an external 12v power supply.\
<img src="https://cdn-learn.adafruit.com/assets/assets/000/002/692/large1024/led_strips_ledstripfet.gif?1448059609" width=600>
