Parse log file for uni project
==============================

Log file looks like this:

```
Transmission ID: 0, Received, Time(secs): 1380368533
--0,327ca0,327cb1,0,0
--1,327ca0,327c80,0,0
--2,327ca0,727ea0,0,0
--3,327ca0,327ca0,1,1
Transmission ID: 1, Received, Time(secs): 1380368534
--0,9be88c,9be884,0,0
--1,9be88c,9be88c,1,1
Transmission ID: 2, Received, Time(secs): 1380368534
--0,1761ba,1760ba,0,0
--1,1761ba,1761ba,1,1
Transmission ID: 3, Received, Time(secs): 1380368534
--0,f0e20c,f0e20c,1,1
Transmission ID: 4, Received, Time(secs): 1380368536
--0,3fdac8,7fcb48,0,0
--1,3fdac8,3fdbc8,0,0
--2,3fdac8,3ddac8,0,0
--3,3fdac8,3f9ac8,0,0
--4,3fdac8,7fd8d8,0,0
--5,3fdac8,3fdac8,1,1
```

Each transmission can take multiple attempts depending on whether the errors were detected. The final two columns of each attempt correspond to whether the codec detected an error and whether there was actually an error, respectively. A `1` indicates success and `0` indicates error.

Decided to parse it using the Nom library in Rust. Parses the text file and produces a vector of structs which can then be analysed with iterators. Currently just counts the average number of attempts per transmission and the residual bit error rate of the system (the rate of undetected errors).

Code was mainly written after one late night and is just for my use so isn't commented as nicely as it would be if it was for someone else to read and enjoy.

