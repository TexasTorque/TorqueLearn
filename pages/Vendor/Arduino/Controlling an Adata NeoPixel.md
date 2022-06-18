# Controlling an Adata NeoPixel

## What you need
* An Arduino (any kind will probably work but we use Arduino UNO)
* Jumper cables
* A 5v power supply (if you have a lot of LEDs and can't just power it off the Arduino)
* The strip
* USB-B cable
* Computer (bruh)

## Why NeoPixel?
<img src="https://cdn-learn.adafruit.com/guides/images/000/000/350/medium800/glamour.jpg" alt="image of the above" width=400/>\
This is the led strip we have chosen to light our robot with for the 2020 season. It features an easy way to individually address the color of each LED, rather than address the whole strip at once. Oh and for bonus points it has an easy-to-use Arduino library that you can download right from the Arduino IDE. It also requires only one data pin connected to the Arduino which isn't that big of a deal but it makes things look less cluttered.

KEEP IN MIND YOU CAN DRIVE THIS LED STRIP (DATA WISE) DIRECTLY OFF OF THE ROBORIO

### Installing the software

To download the library needed to control the Arduino, go to tools > manage libraries > search "Adafruit NeoPixel" and install it. [Here's](https://www.arduino.cc/reference/en/libraries/adafruit-neopixel/) a link to some source on the Arduiuno website to verify you have the right stuff.

### Setting up the hardware
You might need to solder out the power leads into jumpers, so have someone who knows what they are doing do that.

One one side of the led strip, you'll have a power and ground (probably red and black). Attach the red one to the "5v" pin on the Arduino, and the black one to one the "GND" sockets. Then depending on your strip, you might have 1 OR 2 more pins. The black one is just a ground, but as far as I can tell you only need one ground connected, so you can probably just leave it alone. Attach the data pin to the socket labeled "6" on the Arduino, or whatever pin you want just make sure to set it in code. At the end you should have 3 pins connected to the Arduino if you're powering off of the Arduino itself. You can also use the 5v on the VRM to power it.

Use a USB-B cable to connect the Arduino to your computer.

### Programming time
At the top of ANY NeoPixel file you'll need to "import" the library with the following. This gives you all the stuff you need to interface with the strip.
```C++
#include <Adafruit_NeoPixel.h>
```

Also at the top of the file, you're going to want to the following. Think of this as a fancy variable declaration. The compiler goes through and sees wherever "DATA_PIN" appears and replaces it with 6.
```C++
#define DATA_PIN 6 // change this to whatever data pin you need
#define PIXEL_COUNT 60 // number of LEDs on the strip
```

Finally in the global space let's create the strip object as "strip." Create more instances of strip as you need to control more strips separately.
```C++
Adafruit_NeoPixel strip(PIXEL_COUNT, DATA_PIN, NEO_GRB + NEO_KHZ800);
```

The rest of your program will probably look like the following at this point.
```C++
void setup() {

}
void loop() {

}
```

In setup we want to run functions that don't return values we need for the rest of the program's operation. We want to tell Arduino to set the mode of the data pin to output, and run some setup functions for the strip itself. At the most basic level it should look like this.
```C++
void setup() {
    pinMode(DATA_PIN, OUTPUT);
    strip.begin();
    strip.show(); // this tells the library to write whatever changes you've made to the instance "strip" to the physical strip, which as this point is nothing so it will show that
}
```
Then in loop, which runs as fast as the Arduino can manage automatically, we write code that sets the color of strip. For example
```C++
void loop() {
    // iterates through and sets every pixel to green (pixels.Color takes in an RGB value)
    for (int i = 0; i < PIXEL_COUNT; i++) {
        strip.setPixelColor(i, pixels.Color(0, 255, 0)) // set pixel at index i along the strip to color
    }
    strip.show(); // show changes all at once
    delay(5000) // wait 5 seconds
    // same thing but for white
    for (int i = 0; i < PIXEL_COUNT; i++) {
        strip.setPixelColor(i, pixels.Color(255, 255, 255)) 
    }
    strip.show(); // show changes all at once
    delay(5000) // wait 5 seconds
}
```

This code switches the color of the strip every 5 seconds. This doesn't really take advantage of the individual addressability of the LEDs. If you did want to do that you could do a color wipe with something like this, which sets a pixel ever 10th of a second.
```C++
for (int i = 0; i < PIXEL_COUNT; i++) {
    strip.setPixelColor(i, pixels.Color(0, 255, 0)) 
    strip.show(); // show changes one pixel at a time
    delay(100);
}
```

You can find more code examples [here](https://github.com/adafruit/Adafruit_NeoPixel/tree/master/examples)


