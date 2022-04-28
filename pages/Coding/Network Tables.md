# Network Tables

Some basic examples to get you started in various languagues with Network Tables (as a client)

# C++

Including the header:
```c++
#include "networktables/NetworkTable.h"
#include "networktables/NetworkTableInstance.h"
```

Instatiating object and setup:
```c++
auto ntinst = nt::NetworkTableInstance::GetDefault();
// attempts to establish connection to server
// over common IPs based on team number
// use YOUR team number
ntinst.StartClientTeam(1477);
// starting the client
ntinst.StartDSClient();
```

Manipulating a table:
```c++
// grab the table by name
// this is a ptr
// so you have to use
// -> to get members
std::shared_ptr<nt::NetworkTable> table = ntinst.GetTable("tbl");
// put a number to entry!
table->PutNumer("entry_name", 0.5);

// grab an entry from the table
auto entry = table->GetEntry("keyuwu");
// grab a value from the table
// (if it's a boolean)
entry.GetBoolean(false);
```

## Java

Imports:
```java
import edu.wpi.first.networktables.NetworkTable;
import edu.wpi.first.networktables.NetworkTableEntry;
import edu.wpi.first.networktables.NetworkTableInstance;
```

Instantiate and setup:
```java
// should work fine in most cases
NetworkTableInstance ntinst = NetworkTableInstance.getDefault();
// setup/start the client
ntinst.startClientTeam(1477); // YOUR team number
ntinst.startDSClient();
```

Maniupulating a table:
````java
// grab the tble
NetworkTable table = ntinst.getTable("tbl");

NetworkTableEntry one = table.getEntry("entryuwu");

one.getDouble(0.0); // grab whatevers in the entry (as a double)

one.setDouble(10.0); // set it to 10.0, compatible with many Java types

```