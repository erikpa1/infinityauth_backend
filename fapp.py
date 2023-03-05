from flask import Flask

app = Flask("infinityauth", static_url_path='')

@app.get("/")
def get_index():
    return app.send_static_file("index.html")