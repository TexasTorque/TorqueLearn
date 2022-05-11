# Starting Arduino

A basic programming-focused starter's guide to Arduino! You'll need a two terminal button, like shown below, an Arduino, a computer, Arduino jumpers, and some wire strippers.

Oh if you're curious an example of a two-terminal button is shown below.
<img src="https://m.media-amazon.com/images/I/71j74bPl+HL._SL1500_.jpg" alt="image of the above" width=200/>

## Hardware Setup

Get two jumpers, strip one end off of each leaving a decent amount of bare metal. Make sure to leave one male end on each (if they are m/f jumpers). Twist the strands of each jumper tight, and wrap one jumper around the hole in each terminal of the button. Plug one jumper into the 5v pin on the Arduino and the other into any data pin (numbered on the oppsite side of the board) other than 0 or 1.

Plug the Arduino with a USB cable into the computer.

## Software Setup

The Arduino IDE is available [here](https://www.arduino.cc/en/software). Just download the latest version and install normally (you can do that right?). By the way, the language is weird C++ but you probably won't do any OOP since it provides a functional structure with init ("setup") and continuous ("loop"). Even a rudimentary knowledge of C/C++ and you'll be up and coding in no time, and it's easy even if you have no experience in a language like that.

Open the program when you're done installing.

## Programming

When you open your first sketch, you will be met with somthing similar to the following.
```c++


void setup() {

}
void loop() {

}
```
Let's dissect this. The space above setup is the global space. Anything you do there will happen once and any variables declared here will be accessible in both `setup` and `loop`. Here's where you want to declare any variables that you'll be setting later. In `setup` we run stuff that only needs to be run once, and we don't need any data from it for the rest of the runtime of the program. `loop` is where most of the action happens. This is where we'll manage what the Arduino is taking for input/outputting, using everything we did in the global space and `setup`.

Let's code our project. First, we need to set up a global variable to hold the state of the data pin the button is connected. Then we can set this variable later without wasting the time it takes to declare the variable again every loop.
```c++
bool name;
```

Then in `setup`, we need to tell the Arduino to get ready to accept input from the button. We can do that with a call to `pinMode` and some automatically added global values. Oh, we also need to call `Serial.begin(9600)` to tell the Arduino to get ready to output whatever we print to the computer at a baud rate of 9600.
```c++
void setup() {
    // arguments to pinMode are pin number and mode
    // make sure to put whatever pin you hooked the 
    // button up to for the first argument
    pinMode(3, INPUT)
}
```

Then the `loop` function, that is called as fast as the Arduino can manage, we need to `digitalRead` the value of the pin (whether the button is pressed or not) and manage it. We can use that global variable we set earlier.

```c++
void loop() {
    // use YOUR pin #
    name = digitalRead(3);
    // now we can send the value back to the Serial Monitor
    // (in tools) on the computer
    Serial.println(name)
}
```

## Uploading Your Code

First make sure you have the Arduino selected. Go tools > ports and make sure to select COM#. Then, hit the "play" button in the IDE. This will compile and upload your code. Open the Serial Monitor (under tools) and try pressing the button. It should change from `false` to printing `true` when you hold the button down.