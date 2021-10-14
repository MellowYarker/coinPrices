#!/bin/sh
python3 server.py &
echo "server PID: $!"

python3 crawler.py &
echo "Crawler PID: $!"
