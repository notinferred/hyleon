# Serves a web build for the T-02 comparison and saves the results the page posts to /results.
# Usage: python3 -I bench-web.py <folder> <port> <results.json>. It exits once results arrive.
import http.server
import sys

folder, port, out = sys.argv[1], int(sys.argv[2]), sys.argv[3]


class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=folder, **kwargs)

    def do_POST(self):
        body = self.rfile.read(int(self.headers.get("Content-Length", 0)))
        with open(out, "wb") as f:
            f.write(body)
        self.send_response(204)
        self.end_headers()
        self.server.done = True

    def log_message(self, *args):
        pass


server = http.server.HTTPServer(("127.0.0.1", port), Handler)
server.done = False
while not server.done:
    server.handle_request()
