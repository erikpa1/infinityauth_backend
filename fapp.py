from flask import Flask

app = Flask("infinityauth")


@app.get("/")
def get_index():
    return app.send_static_file("index.html")