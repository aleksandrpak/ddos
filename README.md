Booking.com is a big website that receives a lot of traffic. This also makes us a target for DDoS (Distributed Denial of Service) attacks. A simple attach is one that floods the website with meaningless requests.

For this task you need to identify the time interval in which Booking.com is under a DDoS attack.

##Input Format

You’ll have a dataset of time-series data consisting of tuples in an epoch and the number of requests received during that epoch. Within this time-series data there will be multiple attacks when the site is receiving an elevated number of requests.

##Output Format

You should provide the epoch intervals for when each one of the attacks starts and ends.

###Sample Input

    [[<epoch>, <number of requests>],[123456, 45],[123457, 46],[123458, 1000],[123459, 1129],[123460, 999],[123461, 47],[123462, 50],[123463, 67],[123464,35],[123465, 50],[123466, 10000],[123467, 5000],[123468,60]]

###Sample Output

    [[123458, 123460], [123466, 123467]]
