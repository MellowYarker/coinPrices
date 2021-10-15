from multiprocessing import Process, Queue
import os, signal, sys

import crawler, server # my files


def initServer(q):
    # Send the pid back to the parent.
    q.put((os.getpid(), "server"))
    server.runServer()

def initCrawler(q):
    # Send the pid back to the parent.
    q.put((os.getpid(), "crawler"))
    crawler.runCrawler()

if __name__ == "__main__":
    q = Queue()
    serverProc = Process(target=initServer, args=(q,))
    crawlerProc = Process(target=initCrawler, args=(q,))

    serverProc.start()
    crawlerProc.start()

    pids = list()
    while (len(pids) < 2):
        pids.append(q.get())

    print(pids)

    # We put this in the main func so we have access to the pids list.
    def signal_handler(sig, frame):
        print("\nInitiating graceful shutdown of crawler and server.")
        for proc in pids:
            os.kill(proc[0], signal.SIGINT)
        print("Done - exiting.")
        sys.exit(0)

    # Start handling sigint here.
    signal.signal(signal.SIGINT, signal_handler)

    serverProc.join()
    crawlerProc.join()
