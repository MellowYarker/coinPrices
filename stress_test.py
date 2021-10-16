from multiprocessing import Process, Queue
import os, requests, signal, sys, time

import server # my file

def initServer(q):
    # Send the pid back to the parent.
    q.put(os.getpid())
    server.runServer()

if __name__ == "__main__":
    q = Queue()
    serverProc = Process(target=initServer, args=(q,))

    serverProc.start()

    pids = list()
    while (len(pids) < 1):
        pids.append(q.get())

    # We put this in the main func so we have access to the pids list.
    def signal_handler(sig, frame):
        print("\nInitiating graceful shutdown of server.")
        for proc in pids:
            os.kill(proc, signal.SIGINT)
        print("Done - exiting.")
        sys.exit(0)

    # Start handling sigint here.
    signal.signal(signal.SIGINT, signal_handler)

    users = 1000
    users_per_sec_goal = 100
    url = "http://localhost:8000/backend/prices.json"

    start = time.time()

    for i in range(users):
        requests.get(url)

    time_elapsed = time.time() - start
    print(f"\n\nIt took {time_elapsed / (users / users_per_sec_goal)} seconds to serve the data to {users_per_sec_goal} users (simulated {users} users).")

    # Kill the server
    os.kill(pids[0], signal.SIGINT)
