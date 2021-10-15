"""Use instead of `python3 -m http.server` when you need CORS"""

from http.server import HTTPServer, SimpleHTTPRequestHandler
import signal, sys


class CORSRequestHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET')
        self.send_header('Cache-Control', 'no-store, no-cache, must-revalidate')
        return super(CORSRequestHandler, self).end_headers()

# Quiet termination
def signal_handler(sig, frame):
    sys.exit(0)

def runServer():
    # Signal handling
    signal.signal(signal.SIGINT, signal_handler)

    httpd = HTTPServer(('localhost', 8000), CORSRequestHandler)
    httpd.serve_forever()

