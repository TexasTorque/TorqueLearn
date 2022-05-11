# How To Program Autonomous

Programming autonomous is one of the core parts of FRC, and along with that, one of the most complex and challenging. However, if you research enough code and learn enough to understand about the systems that are used at Torque, you would be surprised at how fast and simple it really is.

## Calling Autonomous Mode

When you first select autonomous mode from driver station, a proper Texas Torque repository will first trigger the autoInit() function and then the autoContinous() method. The autoInit() function will call AutoManager.java and more specifically, the chooseCurrentSequence function of AutoManager. That function will then send the auto sequence that you chose from Smart Dashboard to Sequence.java where the individual blocks of the selected sequence will be run, and counted until they are all done. For parsing through the commands that are in the blocks that are in the sequences of your selected sequence, Command.java is used.

## Writing Commands

To write a command, you first need to make a new file inside of the command folder which should be inside of the auto folder. Every command needs to extend the main Command class, and because Command.java has required methods to be overridden, they should be autofilled into your new command file. (Code 1) These methods are init(), continuous(), endCondition(), and end(). In addition to these you should also write out a constructor for your command. If you have variables that you want to be passed into your command from a sequence, you should first define them at the top of your class. In the constructor, pass those variables in the parameters and then set each variable with this.  = the variable inside of the constructor.(Code 2) The next thing to write out is the init(), or initialization function. Override public void init(), and then write out the code that you want to have run one time once the command gets called.(Code 3) Do the same for continuous(), but instead put the code that you want to have continuously happen over and over again inside of it. Do the same for public boolean endCondition and put the if statement inside of it and return the condition. (It should be false until you have a condition that will decide if the command can end). Lastly, override end() and write out the code that should run one time once the command ends.

```java
public class DriveForTime extends Command {}
```

```java
public DriveForTime(double speed, double length) {
    this.speed = speed;
    this.length = length;
}
```

```java
@Override
public void init() {
        start = Timer.getFPGATimestamp();
        Input.getInstance().getDrivebaseInput().setLeftSpeed(speed);
        Input.getInstance().getDrivebaseInput().setRightSpeed(speed);
}
```

## Writing Sequences

To write a sequence, you need to first create a new sequence inside of the sequence folder inside of the auto folder. Define the package at the top of the file and then extend the Sequence class in the class declaration. Just like Command, there are required methods to be overridden, so ensure that you are overriding public void init(). Inside of init, you call blocks that are full of commands. The init() function will run all of the blocks that you call inside of it, and you are able to have multiple blocks in each sequence. After the method is called in init(), define the method outside of init() with a void return type. The first line of the block should contain a new ArrayList of the type Command. The next line can then have the name of the ArrayList “.add(“, and then write out “new”, the name of a command, “(“ the numbers to be passed in as parameters for the command, and then “));”. The next line just needs “addBlock()”, with the name of the ArrayList inside of the parenthesis.(Code 4)

```java
private void initDriveForTimeTest() {
        ArrayList<Command> a = new ArrayList<Command>();  // ArrayList<Command> constructors everything in commands folder
        a.add(new DriveForTime(.5, .25, .25));     // Goes forward at 25% power for .5 second
        addBlock(a);
    }
```

## Credits

Initially written by [Omar Afzal](https://github.com/0mara) in November 2021
