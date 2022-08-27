# Raspberry Pi NetworkTables

A Raspberry Pi is often used on a robot for onboard vision processing. In order to communicate the values that your vision algorithm detects back to your robot code, you can upload the values to NetworkTables and then read them from the robot code.

## WPILib Raspberry Pi OS

While most articles on the internet will tell you to use the WPILib OS for a Raspberry Pi, in my finding its best to avoid it. This is because WPILib disabled WiFi and pip installing because they didn't want there to accidentally be a WiFi connection left enabled on a robot that could interfere with other robots in a competition. However, being unable to pip install will mean that you have to manually upload your libraries through the website interface. In my testing, I ran into a bug that wouldn't let me unzip the tar.gz file that I uplaoded to the pi. This left me with having to download the libary, transfer the library to a flash drive, connect the drive to the pi, mount the drive, move the files, unzip the files, move the files to the python library directory, and even through all of this work we ran into version compatability errors that made it a colossal headache and waste of time. I strongly recommend to just use a stock Raspbian Desktop OS (The GUI will come in handy more than you think). This way you can pip install and not worry about library installation. All that you need to remember to do is setup crontab and disable WiFi (Just click the WiFi icon on the desktop and turn it off!).


### Setting Up Crontab

In order for your code to automatically run on startup, you will need to setup crontab to run the script whenever the pi boots up. To do this, first type 
```python 
crontab -e 
```
You will then be prompted for the editor of your choice. For beginners, I would recommend nano, so type 1. Now, you should see a file at /tmp/crontab.jaPtgW/crontab with a bunch of comments. Go to the very bottom of the file and then type: 
```python
@reboot python3 /home/pi/Desktop/script.py
```

replacing your path and filename accordingly.


## Python NetworkTables

```python
print("About to Connect to Network Tables")
    team = 1477
    ip = "10.14.77.2"
    notified = False
    condition = threading.Condition()
    
    # notify as soon as connection is made
    def connection_listener(connected, info):
        with condition:
            notified = False
            condition.notify()
    
    NetworkTables.initialize(server=ip)
    NetworkTables.addConnectionListener(connection_listener, immediateNotify=True)
    with condition:
        if not notified:
            condition.wait()

    print("Connected to Network Tables")
    ntinst = NetworkTablesInstance.getDefault()
    tb = ntinst.getTable("cube_detection")
    tb.getEntry("yawResidual").forceSetValue(1477)
```


## Java NetworkTables
To read a value:

```java
private NetworkTableInstance nTInstance = NetworkTableInstance.getDefault();
private NetworkTable nTTable = nTInstance.getTable("cube_table");
private Double entry = nTTable.getEntry("yawResidual").getDouble(9999);
```

To push a value:

```java
private NetworkTableInstance nTInstance = NetworkTableInstance.getDefault();
private NetworkTable nTTable = nTInstance.getTable("cube_table");
private Double entry = nTTable.getEntry("yawResidual").getDouble(9999);
entry.forceSetNumber(1234)
```